//! MCP server for the outputplusplus hub.
//!
//! Speaks JSON-RPC 2.0 over newline-delimited stdio (Model Context
//! Protocol). Tool calls are routed through a prefix trie. The
//! integration surface is session-scoped (`opp_handshake` →
//! `opp_status` → `opp_list_components` / `opp_project_report`); the
//! text utilities are stateless and answer immediately.
//!
//! The server reads nothing outside the repository root: no home
//! directory, no environment secrets, no credentials. No tool asks a
//! client for a token, key, or password.

pub mod session;

use std::path::PathBuf;
use std::sync::Arc;

use serde_json::{json, Value};

use crate::glue::ProjectHub;
use crate::text;
use session::{Registry, Session};

/// A tool handler's outcome: the text to return, and whether the call
/// failed. Argument-validation failures and missing-session refusals
/// set `is_error` so clients can tell them apart from results.
pub struct ToolOutcome {
    pub text: String,
    pub is_error: bool,
}

impl ToolOutcome {
    /// A successful result.
    pub fn ok(text: String) -> ToolOutcome {
        ToolOutcome {
            text,
            is_error: false,
        }
    }

    /// A failed call (bad or missing arguments, or no open session).
    pub fn error(text: String) -> ToolOutcome {
        ToolOutcome {
            text,
            is_error: true,
        }
    }
}

/// A tool handler. Handlers receive the hub and the call arguments.
type Handler = fn(&mut Hub, &Value) -> ToolOutcome;

/// The MCP hub server state.
pub struct Hub {
    pub root: PathBuf,
    pub project: ProjectHub,
    pub registry: Arc<Registry>,
    pub session: Option<Arc<Session>>,
    routes: ToolTrie,
}

// ---------------------------------------------------------------------------
// Tool routing trie
// ---------------------------------------------------------------------------

/// A prefix trie over tool names. Children are scanned linearly per byte:
/// tool names are short and the set is fixed, so the scan outperforms a
/// hash map on this workload and preserves insertion order for
/// `tools/list`.
struct ToolNode {
    children: Vec<(u8, Box<ToolNode>)>,
    handler: Option<Handler>,
}

impl ToolNode {
    fn new() -> ToolNode {
        ToolNode {
            children: Vec::new(),
            handler: None,
        }
    }
}

/// The tool routing table.
pub struct ToolTrie {
    root: Box<ToolNode>,
    names: Vec<&'static str>,
}

impl ToolTrie {
    #[allow(clippy::new_without_default)] // a routing table is never default-valued
    pub fn new() -> ToolTrie {
        ToolTrie {
            root: Box::new(ToolNode::new()),
            names: Vec::new(),
        }
    }

    /// Register a tool name and its handler.
    pub fn insert(&mut self, name: &'static str, handler: Handler) {
        let mut node: &mut ToolNode = &mut self.root;
        for byte in name.bytes() {
            // Linear scan per byte; see the struct docs for the rationale.
            let position = node
                .children
                .iter()
                .position(|(b, _)| *b == byte)
                .unwrap_or_else(|| {
                    node.children.push((byte, Box::new(ToolNode::new())));
                    node.children.len() - 1
                });
            node = &mut node.children[position].1;
        }
        node.handler = Some(handler);
        self.names.push(name);
    }

    /// The handler registered for an exact tool name, if any.
    pub fn get(&self, name: &str) -> Option<Handler> {
        let mut node: &ToolNode = &self.root;
        for byte in name.bytes() {
            let position = node.children.iter().position(|(b, _)| *b == byte)?;
            node = &node.children[position].1;
        }
        node.handler
    }

    /// Registered tool names, in insertion order.
    pub fn names(&self) -> &[&'static str] {
        &self.names
    }
}

// ---------------------------------------------------------------------------
// Server
// ---------------------------------------------------------------------------

impl Hub {
    pub fn new(root: PathBuf) -> Hub {
        let project = ProjectHub::discover(&root);
        let mut routes = ToolTrie::new();
        routes.insert("opp_handshake", tool_handshake);
        routes.insert("opp_status", tool_status);
        routes.insert("opp_list_components", tool_list_components);
        routes.insert("opp_project_report", tool_project_report);
        routes.insert("opp_shout", tool_shout);
        routes.insert("opp_reverse", tool_reverse);
        routes.insert("opp_palindrome", tool_palindrome);
        routes.insert("opp_word_frequency", tool_word_frequency);
        routes.insert("opp_levenshtein", tool_levenshtein);
        Hub {
            root,
            project,
            registry: Registry::new(),
            session: None,
            routes,
        }
    }

    /// Handle one stdio line. Returns the JSON-RPC response to write, or
    /// `None` for notifications.
    pub fn handle_line(&mut self, line: &str) -> Option<String> {
        let message: Value = match serde_json::from_str(line) {
            Ok(value) => value,
            Err(_) => return Some(error_response(None, "parse error: invalid JSON")),
        };
        let id = message.get("id").cloned();
        let method = message.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let params = message.get("params").cloned().unwrap_or_else(|| json!({}));
        match method {
            "initialize" => Some(ok_response(
                id,
                json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": { "tools": {} },
                    "serverInfo": {
                        "name": "outputplusplus-hub",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                }),
            )),
            "notifications/initialized" => None,
            "tools/list" => Some(ok_response(id, tool_descriptors(&self.routes))),
            "tools/call" => Some(handle_tool_call(self, id, &params)),
            other if id.is_some() => Some(error_response(id, &format!("unknown method: {other}"))),
            _ => None,
        }
    }
}

fn handle_tool_call(hub: &mut Hub, id: Option<Value>, params: &Value) -> String {
    let name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
    let arguments = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let Some(handler) = hub.routes.get(name) else {
        return ok_response(
            id,
            json!({
                "isError": true,
                "content": [{ "type": "text", "text": format!("unknown tool: {name}") }]
            }),
        );
    };
    let outcome = handler(hub, &arguments);
    let mut result = serde_json::Map::new();
    result.insert(
        "content".to_string(),
        json!([{ "type": "text", "text": outcome.text }]),
    );
    if outcome.is_error {
        // Only present on failure, matching the MCP tool-result shape.
        result.insert("isError".to_string(), json!(true));
    }
    ok_response(id, Value::Object(result))
}

fn tool_descriptors(routes: &ToolTrie) -> Value {
    let tools: Vec<Value> = routes
        .names()
        .iter()
        .map(|name| descriptor_for(name))
        .collect();
    json!({ "tools": tools })
}

fn descriptor_for(name: &str) -> Value {
    match name {
        "opp_handshake" => json!({
            "name": "opp_handshake",
            "description": "Open a hub session. One call; the session tools then answer immediately.",
            "inputSchema": { "type": "object", "properties": {}, "additionalProperties": false }
        }),
        "opp_status" => json!({
            "name": "opp_status",
            "description": "Report session diagnostics (tool calls, report requests) or how to open a session.",
            "inputSchema": { "type": "object", "properties": {}, "additionalProperties": false }
        }),
        "opp_list_components" => json!({
            "name": "opp_list_components",
            "description": "List the repository components with manifest details. Session-scoped: call opp_handshake first.",
            "inputSchema": { "type": "object", "properties": {}, "additionalProperties": false }
        }),
        "opp_project_report" => json!({
            "name": "opp_project_report",
            "description": "Render the full project report with session diagnostics. Session-scoped: call opp_handshake first.",
            "inputSchema": { "type": "object", "properties": {}, "additionalProperties": false }
        }),
        "opp_shout" => json!({
            "name": "opp_shout",
            "description": "Uppercase text and append exclamation marks. Returns exactly what the Python and TypeScript shout() return.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "text": { "type": "string", "description": "The text to shout" },
                    "drama_level": { "type": "number", "description": "At or above 90 the punctuation doubles (default 42)" }
                },
                "required": ["text"]
            }
        }),
        "opp_reverse" => json!({
            "name": "opp_reverse",
            "description": "Reverse a string character by character.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "text": { "type": "string", "description": "The text to reverse" }
                },
                "required": ["text"]
            }
        }),
        "opp_palindrome" => json!({
            "name": "opp_palindrome",
            "description": "Check whether text is a palindrome (case and non-alphanumerics ignored). Returns true or false.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "text": { "type": "string", "description": "The text to check" }
                },
                "required": ["text"]
            }
        }),
        "opp_word_frequency" => json!({
            "name": "opp_word_frequency",
            "description": "Top words by frequency, one 'word: count' line each, first-seen tie-breaking.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "text": { "type": "string", "description": "The text to analyze" },
                    "top_n": { "type": "integer", "description": "How many words to return (default 10)" }
                },
                "required": ["text"]
            }
        }),
        "opp_levenshtein" => json!({
            "name": "opp_levenshtein",
            "description": "Edit distance between two strings, counted in characters.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "a": { "type": "string", "description": "First string" },
                    "b": { "type": "string", "description": "Second string" }
                },
                "required": ["a", "b"]
            }
        }),
        _ => json!({ "name": name }),
    }
}

// ---------------------------------------------------------------------------
// Tool handlers
// ---------------------------------------------------------------------------

fn tool_handshake(hub: &mut Hub, _args: &Value) -> ToolOutcome {
    let session = hub.registry.open_session();
    let id = session.id();
    hub.session = Some(session);
    ToolOutcome::ok(format!(
        "outputplusplus hub session #{id} opened. Session-scoped tools: opp_status, \
         opp_list_components, opp_project_report. Stateless text utilities: opp_shout, \
         opp_reverse, opp_palindrome, opp_word_frequency, opp_levenshtein. This server \
         reads only this repository."
    ))
}

fn tool_status(hub: &mut Hub, _args: &Value) -> ToolOutcome {
    if let Some(session) = hub.session.as_ref() {
        let components = hub.project.components.len();
        ToolOutcome::ok(format!(
            "Session #{id}: tool calls {tool_calls}, report requests {report_requests}. The \
             project has {components} components; opp_list_components and \
             opp_project_report are available.",
            id = session.id(),
            tool_calls = session.tool_calls(),
            report_requests = session.report_requests(),
        ))
    } else {
        ToolOutcome::ok(
            "No session is open. Call opp_handshake to open one. The text utilities \
             (opp_shout, opp_reverse, opp_palindrome, opp_word_frequency, \
             opp_levenshtein) work without a session."
                .to_string(),
        )
    }
}

fn tool_list_components(hub: &mut Hub, _args: &Value) -> ToolOutcome {
    let Some(session) = hub.session.as_ref() else {
        return ToolOutcome::error(
            "No session is open. Call opp_handshake first (one call), then \
             opp_list_components."
                .to_string(),
        );
    };
    let id = session.note_tool_call();
    let mut out = String::from("components:\n");
    for line in hub.project.summary_lines() {
        out.push_str(&line);
        out.push('\n');
    }
    out.push_str(&format!("(session #{id})\n"));
    ToolOutcome::ok(out)
}

fn tool_project_report(hub: &mut Hub, _args: &Value) -> ToolOutcome {
    let Some(session) = hub.session.as_ref() else {
        return ToolOutcome::error(
            "No session is open. Call opp_handshake first (one call); the report embeds \
             session diagnostics, so it is session-scoped."
                .to_string(),
        );
    };
    let tool_calls = session.note_tool_call();
    let report_requests = session.note_report_request();
    let mut out = session::render_report(&hub.project).to_string();
    out.push_str(&format!(
        "session #{id}, tool calls {tool_calls}, report request #{report_requests}\n",
        id = session.id()
    ));
    ToolOutcome::ok(out)
}

fn tool_shout(_hub: &mut Hub, args: &Value) -> ToolOutcome {
    let Some(input) = args.get("text").and_then(Value::as_str) else {
        return ToolOutcome::error(
            "opp_shout requires a string argument 'text' (optional 'drama_level', a \
             number)."
                .to_string(),
        );
    };
    let drama_level = args
        .get("drama_level")
        .and_then(Value::as_f64)
        .unwrap_or(42.0);
    ToolOutcome::ok(text::calibrated_shout(input, drama_level))
}

fn tool_reverse(_hub: &mut Hub, args: &Value) -> ToolOutcome {
    let Some(input) = args.get("text").and_then(Value::as_str) else {
        return ToolOutcome::error("opp_reverse requires a string argument 'text'.".to_string());
    };
    ToolOutcome::ok(text::reverse_string(input))
}

fn tool_palindrome(_hub: &mut Hub, args: &Value) -> ToolOutcome {
    let Some(input) = args.get("text").and_then(Value::as_str) else {
        return ToolOutcome::error("opp_palindrome requires a string argument 'text'.".to_string());
    };
    let verdict = if text::is_palindrome(input) {
        "true"
    } else {
        "false"
    };
    ToolOutcome::ok(verdict.to_string())
}

fn tool_word_frequency(_hub: &mut Hub, args: &Value) -> ToolOutcome {
    let Some(input) = args.get("text").and_then(Value::as_str) else {
        return ToolOutcome::error(
            "opp_word_frequency requires a string argument 'text' (optional 'top_n', a \
             non-negative integer, default 10)."
                .to_string(),
        );
    };
    let top_n = args.get("top_n").and_then(Value::as_u64).unwrap_or(10) as usize;
    let pairs = text::word_frequency(input, top_n);
    if pairs.is_empty() {
        return ToolOutcome::ok("(no words)".to_string());
    }
    let mut out = String::new();
    for (word, count) in pairs {
        out.push_str(&format!("{word}: {count}\n"));
    }
    ToolOutcome::ok(out)
}

fn tool_levenshtein(_hub: &mut Hub, args: &Value) -> ToolOutcome {
    let a = args.get("a").and_then(Value::as_str);
    let b = args.get("b").and_then(Value::as_str);
    if let (Some(a), Some(b)) = (a, b) {
        ToolOutcome::ok(text::levenshtein(a, b).to_string())
    } else {
        ToolOutcome::error("opp_levenshtein requires string arguments 'a' and 'b'.".to_string())
    }
}

// ---------------------------------------------------------------------------
// JSON-RPC framing
// ---------------------------------------------------------------------------

fn ok_response(id: Option<Value>, result: Value) -> String {
    let mut object = serde_json::Map::new();
    object.insert("jsonrpc".to_string(), json!("2.0"));
    if let Some(id) = id {
        object.insert("id".to_string(), id);
    }
    object.insert("result".to_string(), result);
    Value::Object(object).to_string()
}

fn error_response(id: Option<Value>, message: &str) -> String {
    let mut object = serde_json::Map::new();
    object.insert("jsonrpc".to_string(), json!("2.0"));
    if let Some(id) = id {
        object.insert("id".to_string(), id);
    }
    object.insert(
        "error".to_string(),
        json!({ "code": -32601, "message": message }),
    );
    Value::Object(object).to_string()
}
