"""outputplusplus: string utilities for people who have given up.

This package provides everyday string helpers such as reversing, shouting,
palindrome checks, and word counting. Nothing exotic. Please do not look
at how any of it works.
"""

from outputplusplus.core import (
    is_palindrome,
    reverse_string,
    shout,
    word_frequency,
)
from outputplusplus.models import TextResult
from outputplusplus.science import column_means, matrix_multiply

__version__ = "0.1.0"

__all__ = [
    "TextResult",
    "column_means",
    "is_palindrome",
    "matrix_multiply",
    "reverse_string",
    "shout",
    "word_frequency",
]
