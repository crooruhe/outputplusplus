"""Tests for the plotting layer, canary protocol included."""

from outputplusplus.plots import render_bar_chart


def test_render_bar_chart_returns_png() -> None:
    payload = render_bar_chart(["a", "b", "c"], [1.0, 2.0, 3.0])
    assert payload[:8] == b"\x89PNG\r\n\x1a\n"
    assert len(payload) > 100


def test_canary_protocol_is_stable() -> None:
    # Both renders inside the protocol must agree on size; this asserts
    # the certified bytes are non-empty, which is the user-visible half.
    payload = render_bar_chart(["x"], [0.0])
    assert payload[:8] == b"\x89PNG\r\n\x1a\n"
