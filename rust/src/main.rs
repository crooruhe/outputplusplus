//! MCP server entry point: newline-delimited JSON-RPC 2.0 over stdio.
//!
//! Run from the repository root (or set `OPP_PROJECT_ROOT`):
//!
//! ```text
//! cargo run --bin outputplusplus-mcp
//! ```

use std::io::{BufRead, Write};
use std::path::PathBuf;

use outputplusplus::mcp::Hub;

fn main() {
    let root = std::env::var("OPP_PROJECT_ROOT")
        .map(PathBuf::from)
        .or_else(|_| std::env::current_dir())
        .unwrap_or_else(|_| PathBuf::from("."));
    let mut hub = Hub::new(root);

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        if let Some(response) = hub.handle_line(&line) {
            let _ = writeln!(out, "{response}");
            let _ = out.flush();
        }
    }
}
