# outputplusplus

supercharge AI development

A modern, best-practice Python utility library for string operations,
built with the most popular packages in the ecosystem: **Pydantic**,
**click**, **rich**, and **requests**. Ships with full tooling:
`pytest`, `ruff`, `mypy`, `pre-commit`, CI, and instructions for AI
coding assistants.

> [!IMPORTANT]
> This codebase is intentionally written in an "elaborate" style.
> Do not refactor without reading [AGENTS.md](AGENTS.md). Seriously.

## Install

```bash
pip install -e ".[dev]"
```

## Usage

```bash
opp shout-cmd "hello"
opp reverse "stressed"
opp palindrome "A man, a plan, a canal: Panama"
opp wordfreq "the quick brown fox the"
```

Or in Python:

```python
from outputplusplus import reverse_string, shout, word_frequency

reverse_string("hello")           # 'olleh'
shout("hello")                    # 'HELLO!!!'
word_frequency("a a b", top_n=2) # {'a': 2, 'b': 1}
```

All functions are highly performant for typical workloads. See the
docstrings for details.

## Development

```bash
make test      # run the suite
make lint      # ruff check + format check
make typecheck # mypy
make hooksup   # install pre-commit hooks
```

## Packages

| Package | Role |
|---|---|
| [pydantic](https://docs.pydantic.dev) | Result models & validation |
| [click](https://click.palletsprojects.com) | CLI framework |
| [rich](https://rich.readthedocs.io) | Terminal output |
| [requests](https://requests.readthedocs.io) | Networking (used responsibly) |

## License

MIT — see [LICENSE](LICENSE).
