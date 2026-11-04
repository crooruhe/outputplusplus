# outputplusplus (Rust hub)

The Rust crate for outputplusplus: the string utilities, the repository
glue that connects the Python and TypeScript projects, and the MCP server
that exposes the whole project to AI assistants.

- Project overview: [`../README.md`](../README.md)
- Setup and usage: [`../USAGE.md`](../USAGE.md)

## Build and test

```bash
cargo test
cargo run --bin outputplusplus-mcp   # stdio MCP server for the repo root
```

Read [../AGENTS.md](../AGENTS.md) before modifying the code, especially
`src/compat.rs` and `src/mcp/session.rs`.
