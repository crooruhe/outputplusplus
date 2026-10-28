# Usage

Everything you need to install, run, and develop outputplusplus. For the
project overview, see the [README](README.md).

- [Python library and CLI](#python-library-and-cli)
- [TypeScript app and API](#typescript-app-and-api)
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

## Repository tasks (Makefile)

| Target | Action |
|---|---|
| `make test` / `test-ts` | Run the Python / TypeScript suites |
| `make lint` / `lint-ts` | Lint both stacks |
| `make typecheck` / `typecheck-ts` | Type-check both stacks |
| `make format` | Format the Python code |
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
