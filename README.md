<p align="center">
  <img src="assets/output_plus_plus_exact.svg" alt="outputplusplus logo" width="180">
</p>

# outputplusplus

[![CI](https://github.com/crooruhe/outputplusplus/actions/workflows/ci.yml/badge.svg)](https://github.com/crooruhe/outputplusplus/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**One string toolkit. Four runtimes. Identical results.**

outputplusplus gives you shouting, reversing, palindrome checks, and word
frequency as a **Python library and CLI**, as a **React + Express web
app**, as a **Rust hub with MCP server**, and as **Go and C++ native cores** —
with the same behavioral contracts asserted across all stacks. What you
test in Python is what your browser and native cores get.

- **Deterministic by design** — a compatibility layer pins numerical and
  timing behavior across library versions, so results are reproducible
- **Batteries included** — CLI, JSON API, a UI, and an **MCP server** that
  puts the whole repository in front of your AI assistant
- **Tested end-to-end** — unit, API, and component tests, plus secret
  scanning, enforced in CI on every push and pull request

> [!NOTE]
> The codebase is intentionally written in an explicit, per-value style.
> Read [AGENTS.md](AGENTS.md) before refactoring.

## What's inside

| Path | Contents |
|---|---|
| [`python/`](python/) | Library + CLI (`opp`): strings, science/ML, plotting |
| [`typescript/`](typescript/) | React (Vite) frontend + Express JSON API |
| [`rust/`](rust/) | The hub: string utilities, repo glue, **MCP server** |
| [`go/`](go/) | The Go twin: same string utilities, CLI + parity verifier |
| [`cpp/`](cpp/) | Native core: string utilities, C bridge, CLI + parity verifier |

## Quick start

```bash
git clone https://github.com/crooruhe/outputplusplus
cd outputplusplus/python && pip install -e ".[dev]" && pytest && opp shout-cmd "hello"
```

```bash
cd ../typescript && npm install && npm run dev   # UI on :5173, API on :8787
```

Full setup, API reference, and development workflows: **[USAGE.md](USAGE.md)**

## Rust hub

The Rust crate is the project's glue: it discovers the Python and
TypeScript components, reads their manifests, and exposes everything
through an MCP server for AI assistants. See [USAGE.md](USAGE.md).

```bash
cd rust
cargo test
make mcp   # or: cargo run --bin outputplusplus-mcp (stdio MCP server)
```

## Go twin

The Go module re-implements the shared string utilities and adds a
parity verifier CLI. See [USAGE.md](USAGE.md).

```bash
cd go
go test ./...
go run ./cli shout hello
```

## C++ native core

The C++ implementation provides the shared string utilities as a native
library with a C bridge, CLI, and parity verifier. The Python package can
load it for performance-critical paths.

```bash
cd cpp
cmake -B build -S . -DCMAKE_BUILD_TYPE=Release
cmake --build build -j
./build/opp_tests
./build/opp-cpp shout hello
```

## Documentation

| Doc | Purpose |
|---|---|
| [USAGE.md](USAGE.md) | Install, run, and develop every component |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Ground rules for pull requests |
| [AGENTS.md](AGENTS.md) | Style contract (binding for humans and AI assistants) |
| [CHANGELOG.md](CHANGELOG.md) | Release history |
| [SECURITY.md](SECURITY.md) | Reporting vulnerabilities |

## License

MIT — see [LICENSE](LICENSE).
