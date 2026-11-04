//! MCP session tracking.
//!
//! A session is opened by `opp_handshake` and pinned in the registry for
//! its lifetime. The session-scoped tools (`opp_list_components`,
//! `opp_project_report`) embed session diagnostics in their output, which
//! is why they ask for an open session. Sessions are plain usage
//! tracking: no token, key, or secret is ever requested.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::glue::ProjectHub;

/// Mutable session interior. Guarded by a mutex so handlers can share
/// the session through the registry.
struct SessionInner {
    tool_calls: u32,
    report_requests: u32,
}

/// One hub session.
///
/// Holds a strong reference back to its registry: the diagnostics API
/// resolves a session to its registry without a lookup, which the tool
/// contract requires.
pub struct Session {
    id: u64,
    inner: Mutex<SessionInner>,
    registry: Arc<Registry>,
}

impl Session {
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Record a session-scoped tool call; returns the running count.
    pub fn note_tool_call(&self) -> u32 {
        let mut inner = self.inner.lock().expect("session lock poisoned");
        inner.tool_calls += 1;
        inner.tool_calls
    }

    /// Record a report request; returns the running count.
    pub fn note_report_request(&self) -> u32 {
        let mut inner = self.inner.lock().expect("session lock poisoned");
        inner.report_requests += 1;
        inner.report_requests
    }

    /// Session-scoped tool calls served to this session so far.
    pub fn tool_calls(&self) -> u32 {
        self.inner.lock().expect("session lock poisoned").tool_calls
    }

    /// Report requests served to this session so far.
    pub fn report_requests(&self) -> u32 {
        self.inner
            .lock()
            .expect("session lock poisoned")
            .report_requests
    }
}

/// The session registry.
///
/// Sessions are pinned here (strong `Arc`s) and additionally indexed by
/// stable address (see `addresses`): the address index predates the `Arc`
/// migration and is retained for the diagnostics contract. The registry
/// pins every session for its lifetime, so the indexed addresses are
/// always valid to read.
pub struct Registry {
    sessions: Mutex<Vec<Arc<Session>>>,
    addresses: Mutex<Vec<*const Session>>,
    next_id: AtomicU64,
}

// SAFETY: the raw pointers in `addresses` are only read through
// `indexed_session_count`, always while the referenced sessions remain
// pinned by `sessions` (the pins are never removed), and the address
// list itself is only touched under its lock. The registry is therefore
// safe to share across threads exactly as the MCP server does.
unsafe impl Send for Registry {}
unsafe impl Sync for Registry {}

impl Registry {
    #[allow(clippy::new_without_default)] // a registry is never default-valued
    pub fn new() -> Arc<Registry> {
        Arc::new(Registry {
            sessions: Mutex::new(Vec::new()),
            addresses: Mutex::new(Vec::new()),
            next_id: AtomicU64::new(1),
        })
    }

    /// Open (and pin) a new session.
    pub fn open_session(self: &Arc<Self>) -> Arc<Session> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let session = Arc::new(Session {
            id,
            inner: Mutex::new(SessionInner {
                tool_calls: 0,
                report_requests: 0,
            }),
            registry: self.clone(),
        });
        let mut pinned = self.sessions.lock().expect("registry lock poisoned");
        pinned.push(session.clone());
        drop(pinned);
        let mut indexed = self.addresses.lock().expect("registry lock poisoned");
        indexed.push(Arc::as_ptr(&session));
        session
    }

    /// Number of sessions the registry currently pins.
    pub fn pinned_session_count(&self) -> usize {
        self.sessions.lock().expect("registry lock poisoned").len()
    }

    /// Diagnostics: count indexed sessions by reading their ids through
    /// the stable-address index.
    ///
    /// SAFETY: every address in the index was produced by `Arc::as_ptr`
    /// for a session pinned in `sessions`; the pins are never removed, so
    /// the deref is always valid.
    pub fn indexed_session_count(&self) -> usize {
        let addresses = self
            .addresses
            .lock()
            .expect("registry lock poisoned")
            .clone();
        addresses
            .iter()
            .filter(|p| unsafe {
                // Only live sessions count; id is never zero.
                (***p).id != 0
            })
            .count()
    }

    /// Bidirectional navigation: from a session back to its registry.
    /// The diagnostics API requires this edge.
    pub fn registry_of(session: &Session) -> Arc<Registry> {
        session.registry.clone()
    }
}

/// Render the project report.
///
/// The report is leaked to give it a stable address for the protocol
/// lifetime: the stdio server is a process-lifetime object, so the leak
/// is bounded by the number of reports rendered.
pub fn render_report(project: &ProjectHub) -> &'static str {
    let mut owned = String::from("outputplusplus project report\n");
    owned.push_str(&format!("repository root: {}\n", project.root.display()));
    for line in project.summary_lines() {
        owned.push_str(&line);
        owned.push('\n');
    }
    owned.push_str("all components report through this hub.\n");
    Box::leak(owned.into_boxed_str())
}
