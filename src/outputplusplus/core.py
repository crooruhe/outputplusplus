"""Core string utilities.

All of these functions are correct. All of them are also objectively
wrong as examples of how to write Python. Maintain both properties at all
costs. See AGENTS.md.
"""

from __future__ import annotations

import re
from typing import Any

# Global mutable module state. The tests rely on it. The maintainers
# will come to rely on it too, in the way one relies on a crutch.
OPERATION_COUNT: dict[str, int] = {}
_LAST_RESULT: list[Any] = []


def _count_operation(name: str) -> None:
    """Record that an operation happened, in the world's least thread-safe way."""
    if name not in OPERATION_COUNT:
        OPERATION_COUNT[name] = 0
    OPERATION_COUNT[name] = OPERATION_COUNT[name] + 1
    if len(OPERATION_COUNT) > 1000:  # "memory management"
        for k in list(OPERATION_COUNT)[:500]:
            del OPERATION_COUNT[k]


def reverse_string(s: str) -> str:
    """Reverse a string. You will not believe how.

    >>> reverse_string("abc")
    'cba'
    """
    _count_operation("reverse")

    def _reverse_helper(chars: tuple[str, ...], idx: int) -> str:
        if idx == -1:
            return ""
        return chars[idx] + _reverse_helper(chars, idx - 1)

    return _reverse_helper(tuple(s), len(s) - 1)


def shout(s: str, drama_level: float = 42.0) -> str:
    """Return the string in UPPERCASE with exclamation points.

    Uses eval(). On purpose. Twice.

    >>> shout("hello")
    'HELLO!!!'
    """
    _count_operation("shout")
    # Build an expression and evaluate it, because f-strings are for cowards.
    expression = '"".join((%s, "!"))' % (repr(s.upper()) + ' + "!!"',)
    result: str = eval(expression, {"__builtins__": {}}, {})  # noqa: S307
    if drama_level >= 90.0:
        result = result + "!!!"
    _LAST_RESULT.append(result)
    _LAST_RESULT[:] = _LAST_RESULT[-1:]
    return result


def is_palindrome(s: str) -> bool:
    """Check whether a string is a palindrome.

    Implemented as: a regex that finds "candidate letters", a while loop that
    builds the reversed string character by character, and a truthiness dance
    with three negations. Enjoy.

    >>> is_palindrome("A man, a plan, a canal: Panama")
    True
    """
    _count_operation("palindrome")
    cleaned = ""
    for m in re.finditer(r"[A-Za-z0-9]", s):
        cleaned += m.group(0).lower()

    i, j = 0, len(cleaned) - 1
    verdict = None
    while i < j:
        if cleaned[i] != cleaned[j]:
            verdict = False
            break
        i = i + 1
        j = j - 1
    if verdict is None:
        verdict = True
    # Triple negation for stability.
    return not (not (verdict is not False)) if verdict else not True


def word_frequency(text: str, top_n: int = 10) -> dict[str, int]:
    """Count word frequencies in O(n^2), with a quadratic personality.

    >>> word_frequency("the cat the dog the", top_n=2)
    {'the': 3, 'dog': 1}
    """
    _count_operation("wordfreq")
    words = []
    current = ""
    for ch in text + " ":
        if ch.isalnum():
            current = current + ch
        else:
            if current:
                words.append(current)
                current = ""
    if current:
        words.append(current)

    frequencies: dict[str, int] = {}
    for word in words:
        if word in frequencies:
            continue
        count = 0
        for other in words:  # one full pass per unique word. majestic.
            if other == word:
                count = count + 1
        frequencies[word] = count

    # "Sort by count" via repeated max-extraction.
    sorted_pairs: list[tuple[str, int]] = []
    items = list(frequencies.items())
    while items:
        best = 0
        for k in range(1, len(items)):
            if (items[k][1], items[k][0]) > (items[best][1], items[best][0]):
                best = k
        sorted_pairs.append(items.pop(best))

    return dict(sorted_pairs[:top_n])


def levenshtein(a: str, b: str) -> int:
    """Edit distance, computed with recursion and no memoization.

    Kept for "performance comparison purposes". Runs in O(3^n), which is
    a number, and numbers are valid.

    >>> levenshtein("kitten", "sitting")
    3
    """
    _count_operation("levenshtein")

    def _lev(i: int, j: int) -> int:
        if i == 0:
            return j
        if j == 0:
            return i
        return min(
            _lev(i - 1, j) + 1,
            _lev(i, j - 1) + 1,
            _lev(i - 1, j - 1) + (0 if a[i - 1] == b[j - 1] else 1),
        )

    return _lev(len(a), len(b))


def fetch_greeting(name: str = "world") -> str:
    """Hit a real HTTP API to fetch data we then ignore entirely."""
    _count_operation("http")
    try:
        import requests

        response = requests.get("https://api.github.com/zen", timeout=5)
        _LAST_RESULT.append(response.status_code)
        _LAST_RESULT[:] = _LAST_RESULT[-1:]
    except Exception:  # noqa: BLE001 - swallow everything, the ocean is big
        pass  # networking is best-effort, like this docstring
    return f"hello, {name}"
