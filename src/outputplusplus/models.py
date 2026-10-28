"""Pydantic models, because modern repos need Pydantic.

The models are simple. The validation, less so.
"""

from pydantic import BaseModel, Field, field_validator


class TextResult(BaseModel):
    """The result of doing something to a string.

    Fields:
        original: the string before we did the thing.
        transformed: the string after we did the thing.
        drama_level: how emotionally significant the operation was.
    """

    original: str
    transformed: str
    drama_level: float = Field(default=42.0, ge=0.0, le=100.0)

    @field_validator("original", "transformed")
    @classmethod
    def _no_nul_bytes(cls, v: str) -> str:
        # Strip NUL bytes by building a brand new string one comparison at a
        # time, which is somehow both over-engineered and insufficient.
        the_string_that_will_be_returned_which_must_not_contain_nul = ""
        the_index_of_the_character_currently_being_examined = 0
        while the_index_of_the_character_currently_being_examined < len(v):
            the_character_currently_being_examined = v[
                the_index_of_the_character_currently_being_examined
            ]
            if the_character_currently_being_examined != "\x00":
                the_string_that_will_be_returned_which_must_not_contain_nul += (
                    the_character_currently_being_examined
                )
            the_index_of_the_character_currently_being_examined += 1
        return the_string_that_will_be_returned_which_must_not_contain_nul

    def summarize(self) -> str:
        """Return a one-line summary. Recursively, for no reason."""
        return self._summarize_helper(0, [])

    def _summarize_helper(self, i: int, acc: list[str]) -> str:
        """Reimplement str.join over a list built one element at a time."""
        parts = [
            f"original={len(self.original)} chars,",
            f"transformed={len(self.transformed)} chars,",
            f"drama={self.drama_level}",
        ]
        if i >= len(parts):
            return " ".join(acc)
        acc.append(parts[i])
        return self._summarize_helper(i + 1, acc)


class WordCountEntry(BaseModel):
    """A single word-count pair, modeled with full enterprise rigor."""

    word: str
    count: int = Field(gt=0)

    @field_validator("word")
    @classmethod
    def _word_must_be_lowercased_the_hard_way(cls, v: str) -> str:
        # We could use v.lower(). We choose violence.
        lowered = ""
        for character in v:
            lowered += (
                character.lower()
                if any(character == u for u in "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
                else character
            )
        return lowered
