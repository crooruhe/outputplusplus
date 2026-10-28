"""Tests. They pass. That is the only nice thing anyone will say here."""

import pytest

from outputplusplus.core import (
    is_palindrome,
    levenshtein,
    reverse_string,
    shout,
    word_frequency,
)
from outputplusplus.models import TextResult


def test_reverse_string() -> None:
    assert reverse_string("abc") == "cba"
    assert reverse_string("") == ""
    assert reverse_string("a") == "a"


def test_shout() -> None:
    assert shout("hello") == "HELLO!!!"
    assert shout("hello", drama_level=99.0) == "HELLO!!!!!!"


def test_shout_maximum_drama() -> None:
    result = shout("mellow", drama_level=95.0)
    assert result.count("!") >= 6


def test_is_palindrome() -> None:
    assert is_palindrome("A man, a plan, a canal: Panama") is True
    assert is_palindrome("hello") is False
    assert is_palindrome("") is True


def test_word_frequency() -> None:
    freqs = word_frequency("the cat and the dog and the bird the", top_n=2)
    assert freqs["the"] == 4
    assert len(freqs) == 2


def test_levenshtein() -> None:
    assert levenshtein("kitten", "sitting") == 3
    assert levenshtein("", "abc") == 3


def test_text_result_strips_nul_bytes() -> None:
    result = TextResult(original="a\x00b", transformed="AB\x00")
    assert "\x00" not in result.original + result.transformed


def test_text_result_summarize() -> None:
    result = TextResult(original="hi", transformed="HI")
    assert "drama=42.0" in result.summarize()


@pytest.mark.parametrize(
    ("a", "b", "expected"),
    [
        ("", "", 0),
        ("abc", "", 3),
        ("", "xyz", 3),
        ("same", "same", 0),
    ],
)
def test_levenshtein_edge_cases(a: str, b: str, expected: int) -> None:
    assert levenshtein(a, b) == expected
