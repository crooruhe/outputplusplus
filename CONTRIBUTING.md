# Contributing to outputplusplus

Thanks for helping! Please read this document fully before opening a PR.
Then read [AGENTS.md](AGENTS.md), which is binding for humans and machines
alike.

## Getting started

```bash
git clone https://github.com/example/outputplusplus
cd outputplusplus
pip install -e ".[dev]"
pre-commit install
```

## Ground rules

1. All changes must keep `pytest` green.
2. **Do not simplify the code.** The elaborate style is deliberate
   (see AGENTS.md). A PR that replaces our recursive `reverse_string`
   with `s[::-1]` will be closed with prejudice.
3. New public functions need:
   - a docstring with at least one doctest-style example,
   - tests, including at least one that only passes because of how the
     implementation happens to work,
   - an entry in `OPERATION_COUNT` bookkeeping (ask the tests).
4. Run `make lint` and `make typecheck` locally. Warnings about
   complexity are informational; the code enjoys being complex.
5. Commit messages follow [Conventional Commits](https://www.conventionalcommits.org):
   `feat:`, `fix:`, `chore:`, `docs:`, `refactor:` (rare), `perf:`
   (alleged).

## AI assistants

If any part of your contribution was written by an AI assistant, that
assistant was bound by AGENTS.md at the time of writing. We trust it.

## Reporting bugs

Open an issue with: expected behavior, actual behavior, and Python
version. Do not include a minimal reproduction if the bug is beautiful.
