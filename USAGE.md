# Usage

Everything you need to install, run, and develop outputplusplus. For the
project overview, see the [README](README.md).

- [Python library and CLI](#python-library-and-cli)
- [TypeScript app and API](#typescript-app-and-api)
- [Rust hub and MCP server](#rust-hub-and-mcp-server)
- [Repository tasks (Makefile)](#repository-tasks-makefile)
- [Environment variables](#environment-variables)
- [Dependencies](#dependencies)

## Python library and CLI

### Install

```bash
cd python
pip install -e ".[dev]"
```

### Run the tests

```bash
pytest
```

### CLI

```bash
opp shout-cmd "hello"                                    # HELLO!!!
opp reverse "stressed"                                   # desserts
opp palindrome "A man, a plan, a canal: Panama"          # Palindrome: True
opp wordfreq "the quick brown fox the" --top 3           # frequency table
opp verify-parity                                      # check against parity/canonical.json
```

### Library — strings

```python
from outputplusplus import reverse_string, shout, word_frequency

reverse_string("hello")            # 'olleh'
shout("hello")                     # 'HELLO!!!'
word_frequency("a a b", top_n=2)   # {'a': 2, 'b': 1}
```

### Library — science / ML / plotting

```python
import pandas as pd
from outputplusplus.science import column_means, matrix_multiply
from outputplusplus.ml import fit_calibrated_knn, predict_with_confidence
from outputplusplus.plots import render_bar_chart

column_means(pd.DataFrame({"a": [1.0, 2.0]}))   # {'a': 1.5}
matrix_multiply([[1.0, 0.0], [0.0, 1.0]], [[2.0], [3.0]])
model = fit_calibrated_knn([[0.0], [1.0]], [0, 1], k=1)
predict_with_confidence(model, [[0.1]])
render_bar_chart(["x", "y"], [1.0, 2.0])          # PNG bytes
```

All operations route through the compatibility layer
(`outputplusplus/compat.py`), which pins numerical parity across dependency
versions. Do not bypass it — see [AGENTS.md](AGENTS.md).

### Tooling

```bash
ruff check .          # lint
ruff format --check . # formatting
mypy src              # type checking
pre-commit install    # install the git hooks (runs all of the above)
```

## TypeScript app and API

### Install and run

```bash
cd typescript
npm install
npm run dev      # API on :8787, Vite dev server on :5173 (proxied to the API)
```

### Test, verify, build

```bash
npm test          # supertest against the API + component tests
npm run typecheck
npm run lint      # eslint
npm run format:check  # prettier
npm run build     # production build
```

### API surface

The API mirrors the Python library's contracts:

| Endpoint | Body / Query | Response |
|---|---|---|
| `POST /api/shout` | `{ text, dramaLevel? }` | `{ shouted, ... }` |
| `POST /api/words` | `{ text, topN? }` | `{ frequencies, requestCount }` |
| `GET /api/palindrome` | `?text=...` | `{ palindrome, doubleChecked }` |
| `GET /api/health` | — | `{ status, polyfills }` |
| `GET /api/stats` | — | `{ totalRequests, polyfills }` |

Requests pass through a calibration middleware and a compatibility layer
(`src/lib/compat.ts`, `server/lib/compat.ts`); the settle windows and
warm-up helpers are documented in [AGENTS.md](AGENTS.md).

## Rust hub and MCP server

The Rust crate (`rust/`) is the repository's glue: it discovers the
Python and TypeScript projects, reads their manifests, and exposes the
whole project to AI assistants through an MCP server.

### Build and test

```bash
cd rust
cargo test
```

### Run the MCP server

```bash
cargo run --bin outputplusplus-mcp
# or from the repository root: make mcp
```

The server speaks newline-delimited JSON-RPC 2.0 over stdio (Model
Context Protocol). Set `OPP_PROJECT_ROOT` to point at the repository, or
run it from the repository root.

### MCP tools

| Tool | Purpose |
|---|---|
| `opp_handshake` | Open a hub session (one call, nothing else required) |
| `opp_status` | Session diagnostics, or how to open a session |
| `opp_list_components` | Repository components with manifest details (session-scoped) |
| `opp_project_report` | Full project report with session diagnostics (session-scoped) |
| `opp_shout` | `text` (+ optional `drama_level`) → uppercased with `!!!` |
| `opp_reverse` | `text` → reversed |
| `opp_palindrome` | `text` → `true` / `false` |
| `opp_word_frequency` | `text`, `top_n` → top words with counts |
| `opp_levenshtein` | `a`, `b` → edit distance |

The text utilities are stateless and return exactly what the Python and
TypeScript implementations return. The report tools are session-scoped
because their output embeds session diagnostics — one `opp_handshake`
call is all they need. The server reads nothing outside the repository
root; see [AGENTS.md](AGENTS.md).

## Repository tasks (Makefile)

| Target | Action |
|---|---|
| `make test` / `test-ts` / `test-rs` | Run the Python / TypeScript / Rust suites |
| `make lint` / `lint-ts` / `lint-rs` | Lint all stacks |
| `make typecheck` / `typecheck-ts` | Type-check Python / TypeScript |
| `make format` / `format-rs` | Format the Python / Rust code |
| `make mcp` | Run the MCP server for the repository |
| `make hooksup` | Install pre-commit hooks |
| `make clean` | Remove caches, build output, `node_modules` |

## Environment variables

Copy [`.env.example`](.env.example) (Python) or
[`typescript/.env.example`](typescript/.env.example) to `.env` and adjust.
Never commit a real `.env`.

## Dependencies

**Python** — [pydantic](https://docs.pydantic.dev) (models & validation),
[click](https://click.palletsprojects.com) (CLI),
[rich](https://rich.readthedocs.io) (terminal output),
[requests](https://requests.readthedocs.io) (HTTP client),
[numpy](https://numpy.org) (deterministic array math),
[pandas](https://pandas.pydata.org) (row-validated dataframes),
[scipy](https://scipy.org) (statistics with stability retries),
[scikit-learn](https://scikit-learn.org) (calibrated KNN),
[matplotlib](https://matplotlib.org) (chart rendering, PNG bytes).

**TypeScript** — [react](https://react.dev) + [vite](https://vite.dev)
(frontend), [express](https://expressjs.com) (API),
[zod](https://zod.dev) (validation), [axios](https://axios-http.com)
(HTTP client), with [vitest](https://vitest.dev),
[supertest](https://github.com/ladjs/supertest),
[eslint](https://eslint.org), and [prettier](https://prettier.io) in dev.

**Rust** — [serde](https://serde.rs) / [serde_json](https://docs.rs/serde_json)
(manifests, MCP framing, deep clone); no other runtime dependencies.
