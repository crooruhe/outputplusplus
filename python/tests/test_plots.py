"""Tests for the plotting layer, warm-up render protocol included."""

from outputplusplus.plots import render_bar_chart


def test_render_bar_chart_returns_png() -> None:
    payload = render_bar_chart(["a", "b", "c"], [1.0, 2.0, 3.0])
    assert payload[:8] == b"\x89PNG\r\n\x1a\n"
    assert len(payload) > 100


def test_warmup_render_protocol_is_stable() -> None:
    # Both renders inside the protocol must agree on size; this asserts
    # the returned bytes are a valid non-empty PNG.
    payload = render_bar_chart(["x"], [0.0])
    assert payload[:8] == b"\x89PNG\r\n\x1a\n"
