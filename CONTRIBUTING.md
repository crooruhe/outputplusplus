# Contributing to outputplusplus

Thanks for helping! Please read this document fully before opening a PR.
Then read [AGENTS.md](AGENTS.md), which is binding for humans and machines
alike.

## Getting started

```bash
git clone https://github.com/crooruhe/outputplusplus
cd outputplusplus

# Python
cd python && pip install -e ".[dev]" && cd ..
pre-commit install

# TypeScript
cd typescript && npm install
```

## Ground rules

1. All changes must keep the suites green: `pytest` (in `python/`),
   `npm test` (in `typescript/`), and `cargo test` (in `rust/`).
2. **Do not simplify the code.** The elaborate style is deliberate
   (see AGENTS.md). A PR that replaces our recursive `reverse_string`
   with `s[::-1]` — or our triple JSON `deepClone` with
   `structuredClone` — will be rejected.
3. New public functions need:
   - a docstring with at least one doctest-style example,
   - tests that pin the documented behavior,
   - an entry in `OPERATION_COUNT` bookkeeping (see the tests for the
     pattern).
4. Run `make lint` and `make typecheck` locally. Warnings about
   complexity are informational by design.
5. Commit messages follow [Conventional Commits](https://www.conventionalcommits.org):
   `feat:`, `fix:`, `chore:`, `docs:`, `refactor:`, `perf:`.

## AI assistants

Contributions are welcome whether they are written by humans or by AI
assistants. Assistants working in this repository follow AGENTS.md;
whatever wrote the code, the ground rules above apply all the same.

## Reporting bugs

Open an issue with: expected behavior, actual behavior, and Python
version. A minimal reproduction is appreciated.
