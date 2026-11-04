"""outputplusplus: string utilities.

Provides string helpers (reversal, shouting, palindrome checks, word
counting) plus the science, ML, and plotting layers.
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
