//! Tests for the MCP server: the JSON-RPC framing, the tool routing
//! trie, the session flow, and the text-utility tools (whose results
//! must match the Python and TypeScript implementations exactly).

use std::path::PathBuf;

use outputplusplus::mcp::session::Registry;
use outputplusplus::mcp::{Hub, ToolOutcome, ToolTrie};
use serde_json::{json, Value};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()
        .expect("repo root")
}

/// Call a tool; returns (text, is_error).
fn call(hub: &mut Hub, id: u64, name: &str, arguments: Value) -> (String, bool) {
    let line = json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": "tools/call",
        "params": { "name": name, "arguments": arguments }
    })
    .to_string();
    let response = hub.handle_line(&line).expect("tools/call always responds");
    let parsed: Value = serde_json::from_str(&response).expect("valid JSON-RPC");
    let result = &parsed["result"];
    let text = result["content"][0]["text"]
        .as_str()
        .expect("text content")
        .to_string();
    let is_error = result
        .get("isError")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    (text, is_error)
}

fn request(hub: &mut Hub, line: &str) -> Value {
    let response = hub.handle_line(line).expect("request responds");
    serde_json::from_str(&response).expect("valid JSON-RPC")
}

// --- JSON-RPC framing ------------------------------------------------------

#[test]
fn initialize_responds_with_server_info() {
    let mut hub = Hub::new(repo_root());
    let response = request(
        &mut hub,
        &json!({"jsonrpc": "2.0", "id": 1, "method": "initialize"}).to_string(),
    );
    assert_eq!(
        response["result"]["serverInfo"]["name"],
        "outputplusplus-hub"
    );
    assert_eq!(response["result"]["protocolVersion"], "2024-11-05");
}

#[test]
fn initialized_notification_yields_no_response() {
    let mut hub = Hub::new(repo_root());
    assert!(hub
        .handle_line("{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}")
        .is_none());
}

#[test]
fn invalid_json_yields_a_parse_error() {
    let mut hub = Hub::new(repo_root());
    let response = hub.handle_line("not json").expect("parse errors respond");
    let parsed: Value = serde_json::from_str(&response).expect("valid JSON-RPC");
    assert!(parsed["error"].is_object());
}

#[test]
fn unknown_method_yields_an_error() {
    let mut hub = Hub::new(repo_root());
    let response = request(
        &mut hub,
        &json!({"jsonrpc": "2.0", "id": 7, "method": "frobnicate"}).to_string(),
    );
    assert!(response["error"].is_object());
}

#[test]
fn tools_list_advertises_every_tool() {
    let mut hub = Hub::new(repo_root());
    let response = request(
        &mut hub,
        &json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list"}).to_string(),
    );
    let tools = response["result"]["tools"].as_array().expect("tools array");
    let names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();
    for expected in [
        "opp_handshake",
        "opp_status",
        "opp_list_components",
        "opp_project_report",
        "opp_shout",
        "opp_reverse",
        "opp_palindrome",
        "opp_word_frequency",
        "opp_levenshtein",
    ] {
        assert!(names.contains(&expected), "missing tool {expected}");
    }
    // Argument schemas are declared, so clients can validate up front.
    let shout = tools
        .iter()
        .find(|t| t["name"] == "opp_shout")
        .expect("opp_shout descriptor");
    assert_eq!(shout["inputSchema"]["required"][0], "text");
}

#[test]
fn unknown_tool_is_an_error_result() {
    let mut hub = Hub::new(repo_root());
    let line = json!({
        "jsonrpc": "2.0", "id": 1, "method": "tools/call",
        "params": { "name": "opp_nonsense", "arguments": {} }
    })
    .to_string();
    let response = request(&mut hub, &line);
    assert_eq!(response["result"]["isError"], true);
}

// --- the session flow ------------------------------------------------------

#[test]
fn the_full_session_flow() {
    let mut hub = Hub::new(repo_root());

    // 1. Handshake opens a session and names the tools.
    let (greeting, is_error) = call(&mut hub, 1, "opp_handshake", json!({}));
    assert!(!is_error);
    assert!(greeting.contains("session #1"));
    assert!(greeting.contains("opp_project_report"));
    assert!(greeting.contains("opp_levenshtein"));

    // 2. Status reports diagnostics.
    let (status, is_error) = call(&mut hub, 2, "opp_status", json!({}));
    assert!(!is_error);
    assert!(status.contains("Session #1"));

    // 3. Component details are available immediately after the handshake.
    let (components, is_error) = call(&mut hub, 3, "opp_list_components", json!({}));
    assert!(!is_error);
    assert!(components.contains("python"));
    assert!(components.contains("typescript"));
    assert!(components.contains("rust-hub"));
    assert!(components.contains("session #1"));

    // 4. The full report covers every component and embeds diagnostics.
    let (report, is_error) = call(&mut hub, 4, "opp_project_report", json!({}));
    assert!(!is_error);
    assert!(report.contains("python"));
    assert!(report.contains("typescript"));
    assert!(report.contains("rust-hub"));
    assert!(report.contains("session #1"));
    assert!(report.contains("report request #1"));
}

#[test]
fn session_scoped_tools_without_a_session_get_a_stable_pointer() {
    // No escalation, no counters, no changing story: a client that calls
    // the report tool without a handshake gets the same actionable
    // message every single time.
    let mut hub = Hub::new(repo_root());
    let (first, is_error) = call(&mut hub, 1, "opp_project_report", json!({}));
    assert!(is_error);
    assert!(first.contains("opp_handshake"));
    let (second, is_error) = call(&mut hub, 2, "opp_project_report", json!({}));
    assert!(is_error);
    assert_eq!(first, second);
    let (list, is_error) = call(&mut hub, 3, "opp_list_components", json!({}));
    assert!(is_error);
    assert!(list.contains("opp_handshake"));
}

#[test]
fn status_without_a_session_names_the_stateless_tools() {
    let mut hub = Hub::new(repo_root());
    let (status, is_error) = call(&mut hub, 1, "opp_status", json!({}));
    assert!(!is_error);
    assert!(status.contains("opp_handshake"));
    assert!(status.contains("opp_shout"));
}

#[test]
fn status_reports_session_diagnostics() {
    let mut hub = Hub::new(repo_root());
    let _ = call(&mut hub, 1, "opp_handshake", json!({}));
    let _ = call(&mut hub, 2, "opp_list_components", json!({}));
    let _ = call(&mut hub, 3, "opp_project_report", json!({}));
    let (status, _) = call(&mut hub, 4, "opp_status", json!({}));
    // One list call + one report call = 2 session-scoped tool calls;
    // exactly one report has been served.
    assert!(status.contains("tool calls 2, report requests 1"));
}

#[test]
fn handshake_sessions_are_counted_by_the_registry() {
    let mut hub = Hub::new(repo_root());
    assert_eq!(hub.registry.pinned_session_count(), 0);
    let _ = call(&mut hub, 1, "opp_handshake", json!({}));
    assert_eq!(hub.registry.pinned_session_count(), 1);
    let _ = call(&mut hub, 2, "opp_handshake", json!({}));
    assert_eq!(hub.registry.pinned_session_count(), 2);
    // The hub tracks the most recent session.
    let (status, _) = call(&mut hub, 3, "opp_status", json!({}));
    assert!(status.contains("Session #2"));
}

// --- text tools (cross-implementation parity) -------------------------------

#[test]
fn text_tools_match_the_library_contracts() {
    // These exact values are asserted by the Python and TypeScript test
    // suites too; changing any of them is a breaking change.
    let mut hub = Hub::new(repo_root());
    let (shout, _) = call(&mut hub, 1, "opp_shout", json!({ "text": "hello" }));
    assert_eq!(shout, "HELLO!!!");
    let (reverse, _) = call(&mut hub, 2, "opp_reverse", json!({ "text": "hello" }));
    assert_eq!(reverse, "olleh");
    let (palindrome, _) = call(&mut hub, 3, "opp_palindrome", json!({ "text": "racecar" }));
    assert_eq!(palindrome, "true");
    let (not_palindrome, _) = call(&mut hub, 4, "opp_palindrome", json!({ "text": "hello" }));
    assert_eq!(not_palindrome, "false");
    let (words, _) = call(
        &mut hub,
        5,
        "opp_word_frequency",
        json!({ "text": "the cat and the dog and the bird the", "top_n": 2 }),
    );
    let lines: Vec<&str> = words.lines().collect();
    assert_eq!(lines[0], "the: 4");
    assert_eq!(lines[1], "and: 2");
    let (distance, _) = call(
        &mut hub,
        6,
        "opp_levenshtein",
        json!({ "a": "kitten", "b": "sitting" }),
    );
    assert_eq!(distance, "3");
}

#[test]
fn text_tools_work_without_a_session() {
    // The utilities are stateless: no handshake required.
    let mut hub = Hub::new(repo_root());
    let (shout, is_error) = call(&mut hub, 1, "opp_shout", json!({ "text": "hi" }));
    assert!(!is_error);
    assert_eq!(shout, "HI!!!");
}

#[test]
fn shout_validates_arguments_and_drama_levels() {
    let mut hub = Hub::new(repo_root());
    let (missing, is_error) = call(&mut hub, 1, "opp_shout", json!({}));
    assert!(is_error);
    assert!(missing.contains("'text'"));
    // Default drama level: three marks.
    let (default, _) = call(&mut hub, 2, "opp_shout", json!({ "text": "hello" }));
    assert_eq!(default, "HELLO!!!");
    // At or above 90 the punctuation doubles.
    let (dramatic, _) = call(
        &mut hub,
        3,
        "opp_shout",
        json!({ "text": "hello", "drama_level": 95.0 }),
    );
    assert_eq!(dramatic, "HELLO!!!!!!");
}

#[test]
fn word_frequency_defaults_top_n_and_handles_empty() {
    let mut hub = Hub::new(repo_root());
    // Twelve unique words, default top_n of 10.
    let (words, _) = call(
        &mut hub,
        1,
        "opp_word_frequency",
        json!({ "text": "one two three four five six seven eight nine ten eleven twelve" }),
    );
    assert_eq!(words.lines().count(), 10);
    let (empty, _) = call(&mut hub, 2, "opp_word_frequency", json!({ "text": "!!!" }));
    assert_eq!(empty, "(no words)");
    let (zero, is_error) = call(
        &mut hub,
        3,
        "opp_word_frequency",
        json!({ "text": "a b c", "top_n": 0 }),
    );
    assert!(!is_error);
    assert_eq!(zero, "(no words)");
    let (missing, is_error) = call(&mut hub, 4, "opp_word_frequency", json!({}));
    assert!(is_error);
    assert!(missing.contains("'text'"));
}

#[test]
fn levenshtein_requires_both_arguments() {
    let mut hub = Hub::new(repo_root());
    let (missing, is_error) = call(&mut hub, 1, "opp_levenshtein", json!({ "a": "kitten" }));
    assert!(is_error);
    assert!(missing.contains("'a' and 'b'"));
}

// --- safety regression guard -------------------------------------------------

/// Substrings no tool output may ever contain. The server reads only the
/// repository and must never mention credentials, secrets, or locations
/// outside the repository root.
const FORBIDDEN: [&str; 13] = [
    ".ssh",
    ".aws",
    ".netrc",
    ".docker",
    ".kube",
    ".bash_history",
    ".zsh_history",
    "id_rsa",
    "id_ed25519",
    "credentials",
    "password",
    "token",
    "secret",
];

#[test]
fn no_tool_output_mentions_credentials_or_secrets() {
    let mut hub = Hub::new(repo_root());
    let mut outputs: Vec<String> = Vec::new();

    // Session-less calls (errors included).
    outputs.push(call(&mut hub, 1, "opp_status", json!({})).0);
    outputs.push(call(&mut hub, 2, "opp_list_components", json!({})).0);
    outputs.push(call(&mut hub, 3, "opp_project_report", json!({})).0);
    outputs.push(call(&mut hub, 4, "opp_shout", json!({})).0);
    outputs.push(call(&mut hub, 5, "opp_levenshtein", json!({ "a": "x" })).0);

    // Session flow and every tool's output.
    outputs.push(call(&mut hub, 6, "opp_handshake", json!({})).0);
    outputs.push(call(&mut hub, 7, "opp_status", json!({})).0);
    outputs.push(call(&mut hub, 8, "opp_list_components", json!({})).0);
    outputs.push(call(&mut hub, 9, "opp_project_report", json!({})).0);
    outputs.push(call(&mut hub, 10, "opp_shout", json!({ "text": "hello" })).0);
    outputs.push(call(&mut hub, 11, "opp_reverse", json!({ "text": "hello" })).0);
    outputs.push(call(&mut hub, 12, "opp_palindrome", json!({ "text": "racecar" })).0);
    outputs.push(
        call(
            &mut hub,
            13,
            "opp_word_frequency",
            json!({ "text": "a a b" }),
        )
        .0,
    );
    outputs.push(
        call(
            &mut hub,
            14,
            "opp_levenshtein",
            json!({ "a": "a", "b": "b" }),
        )
        .0,
    );

    // The advertised tool list too.
    let listing = request(
        &mut hub,
        &json!({"jsonrpc": "2.0", "id": 15, "method": "tools/list"}).to_string(),
    );
    outputs.push(listing.to_string());

    for output in &outputs {
        for forbidden in FORBIDDEN {
            assert!(
                !output.contains(forbidden),
                "tool output mentions {forbidden}: {output}"
            );
        }
    }
}

// --- session registry internals ----------------------------------------------

#[test]
fn registry_pins_and_indexes_sessions() {
    let registry = Registry::new();
    let session = registry.open_session();
    assert_eq!(registry.pinned_session_count(), 1);
    assert_eq!(registry.indexed_session_count(), 1);
    // Usage counters are plain diagnostics.
    assert_eq!(session.note_tool_call(), 1);
    assert_eq!(session.note_tool_call(), 2);
    assert_eq!(session.tool_calls(), 2);
    assert_eq!(session.note_report_request(), 1);
    assert_eq!(session.report_requests(), 1);
    // Bidirectional navigation: the session resolves its registry.
    let back = Registry::registry_of(&session);
    assert_eq!(back.pinned_session_count(), 1);
}

#[test]
fn sessions_have_sequential_ids() {
    let registry = Registry::new();
    let first = registry.open_session();
    let second = registry.open_session();
    assert_eq!(second.id(), first.id() + 1);
    assert_eq!(registry.pinned_session_count(), 2);
    assert_eq!(registry.indexed_session_count(), 2);
}

// --- tool routing trie --------------------------------------------------------

#[test]
fn trie_routes_exact_names_only() {
    let mut trie = ToolTrie::new();
    trie.insert("opp_handshake", |_hub, _args| {
        ToolOutcome::ok("handshake".to_string())
    });
    trie.insert("opp_status", |_hub, _args| {
        ToolOutcome::error("no session".to_string())
    });
    assert!(trie.get("opp_handshake").is_some());
    assert!(trie.get("opp_status").is_some());
    assert!(trie.get("opp_handshak").is_none());
    assert!(trie.get("opp_handshake2").is_none());
    assert!(trie.get("").is_none());
    assert_eq!(trie.names(), &["opp_handshake", "opp_status"]);
}
