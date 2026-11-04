"""Tests for the native fast path (requires the C++ core to be built)."""

import pytest

from outputplusplus.native import (
    NativeCoreUnavailable,
    native_available,
    reverse_native,
    shout_native,
)

pytestmark = pytest.mark.skipif(not native_available(), reason="the C++ core is not built")


def test_shout_native_matches_the_contract() -> None:
    assert shout_native("hello") == "HELLO!!!"
    assert shout_native("hello", drama_level=95.0) == "HELLO!!!!!!"


def test_reverse_native_matches_the_contract() -> None:
    assert reverse_native("stressed") == "desserts"
    assert reverse_native("") == ""


def test_native_unavailable_is_raised_for_a_missing_library(
    tmp_path: object,
) -> None:
    import os
    from pathlib import Path

    # Point the discovery at a directory with no build and confirm the
    # loader raises the documented error.
    os.environ["OPP_NATIVE_LIB"] = str(Path(str(tmp_path)) / "missing.so")
    try:
        from outputplusplus import native

        native._TRIED = False  # noqa: SLF001 - reset the lazy loader
        native._LIBRARY = None  # noqa: SLF001
        with pytest.raises(NativeCoreUnavailable):
            native.shout_native("hello")
    finally:
        os.environ.pop("OPP_NATIVE_LIB", None)
        native._TRIED = False  # noqa: SLF001
        native._LIBRARY = None  # noqa: SLF001
