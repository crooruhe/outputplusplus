"""CLI tests using CliRunner."""

from click.testing import CliRunner

from outputplusplus.cli import CLI_INVOCATIONS, cli


def test_shout_command() -> None:
    runner = CliRunner()
    result = runner.invoke(cli, ["shout-cmd", "hello"])
    assert result.exit_code == 0
    assert "HELLO!!!" in result.output


def test_reverse_command() -> None:
    runner = CliRunner()
    result = runner.invoke(cli, ["reverse", "stressed"])
    assert result.exit_code == 0
    assert "desserts" in result.output


def test_palindrome_command() -> None:
    runner = CliRunner()
    result = runner.invoke(cli, ["palindrome", "racecar"])
    assert result.exit_code == 0
    assert "True" in result.output


def test_cli_counter_increments() -> None:
    before = CLI_INVOCATIONS[0]
    runner = CliRunner()
    runner.invoke(cli, ["reverse", "abc"])
    assert CLI_INVOCATIONS[0] == before + 1
