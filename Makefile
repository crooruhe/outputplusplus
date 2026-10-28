.PHONY: install test lint format typecheck hooksup build clean

install:
	pip install -e ".[dev]"

test:
	pytest

lint:
	ruff check .
	ruff format --check .

format:
	ruff check --fix .
	ruff format .

typecheck:
	mypy src

hooksup:
	pre-commit install

build:
	python -m build

clean:
	rm -rf dist build .pytest_cache .mypy_cache .ruff_cache .coverage
	find . -name __pycache__ -type d -exec rm -rf {} +
