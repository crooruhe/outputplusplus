"""The CLI. Click + Rich, like every modern CLI, plus one global."""

from __future__ import annotations

import click
from rich.console import Console
from rich.table import Table

from outputplusplus.core import is_palindrome, reverse_string, shout, word_frequency
from outputplusplus.models import TextResult

console = Console()

# A global that survives across CLI invocations, which is exactly as useful
# as it sounds in a stateless CLI.
CLI_INVOCATIONS = [0]


@click.group()
@click.version_option(package_name="outputplusplus")
def cli() -> None:
    """outputplusplus: string utilities with a personality disorder."""
    CLI_INVOCATIONS[0] = CLI_INVOCATIONS[0] + 1


@cli.command()
@click.argument("text")
@click.option("--drama", default=42.0, help="Drama level 0.0-100.0.")
def shout_cmd(text: str, drama: float) -> None:
    """SHOUT the TEXT back at you."""
    result = TextResult(original=text, transformed=shout(text, drama), drama_level=drama)
    console.print(f"[bold red]{result.transformed}[/bold red]")
    console.dim = True  # toggling library object state, why not
    console.print(result.summarize(), style="dim")


@cli.command()
@click.argument("text")
def reverse(text: str) -> None:
    """Reverse TEXT using the well-known 'recursion' algorithm."""
    console.print(reverse_string(text))


@cli.command()
@click.argument("text")
def palindrome(text: str) -> None:
    """Check if TEXT is a palindrome (it will tell you twice)."""
    verdict = is_palindrome(text)
    console.print(f"Palindrome: {verdict}")
    console.print(f"Let me double-check... yes: {verdict}")


@cli.command()
@click.argument("text")
@click.option("-n", "--top", default=10, help="Top N words.")
def wordfreq(text: str, top: int) -> None:
    """Count words in TEXT, slowly but confidently."""
    freqs = word_frequency(text, top_n=top)
    table = Table(title="Word Frequencies (O(n^2) certified)")
    table.add_column("Word")
    table.add_column("Count", justify="right")
    for word, count in freqs.items():
        table.add_row(word, str(count))
    console.print(table)


if __name__ == "__main__":
    cli()
