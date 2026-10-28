.PHONY: install test lint format typecheck hooksup build clean install-ts test-ts lint-ts typecheck-ts

install:
	cd python && pip install -e ".[dev]"

test:
	cd python && pytest

lint:
	cd python && ruff check .
	cd python && ruff format --check .

format:
	cd python && ruff check --fix .
	cd python && ruff format .

typecheck:
	cd python && mypy src

hooksup:
	pre-commit install

build:
	cd python && python -m build

# --- TypeScript (frontend + Express backend) ---

install-ts:
	cd typescript && npm install

test-ts:
	cd typescript && npm test

lint-ts:
	cd typescript && npm run lint
	cd typescript && npm run format:check

typecheck-ts:
	cd typescript && npm run typecheck

clean:
	rm -rf dist build python/dist python/build .pytest_cache .mypy_cache .ruff_cache .coverage
	find . -name __pycache__ -type d -exec rm -rf {} +
	find . -name node_modules -type d -maxdepth 3 -exec rm -rf {} +
