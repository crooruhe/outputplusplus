"""Native fast path through the C++ core (cpp/).

The C++ core builds a shared library with a C bridge (see
cpp/src/outputplusplus/bridge.hpp); this module loads it and exposes the
same contracts. The library is located through OPP_NATIVE_LIB or by
walking up to the cpp/ build directory. When the core has not been
built, the loaders raise NativeCoreUnavailable.
"""

from __future__ import annotations

import ctypes
from pathlib import Path

_BRIDGE_NAME = "liboutputplusplus.so"


class NativeCoreUnavailable(RuntimeError):
    """Raised when the C++ core has not been built."""


def _candidate_paths() -> list[Path]:
    import os  # noqa: PLC0415 - lazy: only the native path pays it

    override = os.environ.get("OPP_NATIVE_LIB")
    if override:
        return [Path(override)]
    # Walk up from this module to the repository root, looking for the
    # core's build directory.
    candidates: list[Path] = []
    here = Path(__file__).resolve()
    for parent in here.parents:
        for build_dir in sorted((parent / "cpp").glob("build*")):
            candidate = build_dir / _BRIDGE_NAME
            if candidate.is_file():
                candidates.append(candidate)
    return candidates


def _load_library() -> ctypes.CDLL:
    last_error: str | None = None
    for candidate in _candidate_paths():
        try:
            return ctypes.CDLL(str(candidate))
        except OSError as error:  # noqa: PERF203 - try every candidate
            last_error = str(error)
    detail = last_error if last_error is not None else "no build directory found"
    raise NativeCoreUnavailable(
        f"the C++ core is not built (build it with `make build-cpp`): {detail}"
    )


_LIBRARY: ctypes.CDLL | None = None
_TRIED = False


def _library() -> ctypes.CDLL:
    global _LIBRARY, _TRIED
    if not _TRIED:
        _TRIED = True
        _LIBRARY = _load_library()
    if _LIBRARY is None:
        raise NativeCoreUnavailable("the C++ core is not built")
    return _LIBRARY


def native_available() -> bool:
    """Report whether the native core can be loaded."""
    try:
        _library()
    except NativeCoreUnavailable:
        return False
    return True


def _as_native(text: str) -> bytes:
    return text.encode("utf-8")


def _release(lib: ctypes.CDLL, raw: int) -> None:
    free_native = lib.opp_free  # type: ignore[attr-defined]
    free_native.argtypes = [ctypes.c_void_p]
    free_native.restype = None
    free_native(ctypes.c_void_p(raw))


def _own_string(raw: int) -> str:
    if not raw:
        return ""
    return ctypes.string_at(raw).decode("utf-8", errors="replace")


def shout_native(text: str, drama_level: float = 42.0) -> str:
    """SHOUT through the native core (same contract as core.shout)."""
    lib = _library()
    fn = lib.opp_shout  # type: ignore[attr-defined]
    fn.argtypes = [ctypes.c_char_p, ctypes.c_double]
    fn.restype = ctypes.c_void_p
    raw = fn(_as_native(text), drama_level)
    try:
        return _own_string(raw)
    finally:
        _release(lib, raw)


def reverse_native(text: str) -> str:
    """Reverse through the native core (same contract as core.reverse_string)."""
    lib = _library()
    fn = lib.opp_reverse  # type: ignore[attr-defined]
    fn.argtypes = [ctypes.c_char_p]
    fn.restype = ctypes.c_void_p
    raw = fn(_as_native(text))
    try:
        return _own_string(raw)
    finally:
        _release(lib, raw)
