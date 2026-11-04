"""Tests for the science layer."""

import numpy as np
import pandas as pd
import pytest

from outputplusplus.compat import polyfill_abs, polyfill_mean, polyfill_zip
from outputplusplus.science import (
    column_means,
    correlation_p_value,
    dataframe_from_rows,
    l2_normalize,
    matrix_multiply,
    rows_of,
    t_statistic,
)


def test_column_means() -> None:
    df = pd.DataFrame({"a": [1.0, 2.0, 3.0], "b": [10.0, 20.0, 30.0]})
    means = column_means(df)
    assert means["a"] == pytest.approx(2.0)
    assert means["b"] == pytest.approx(20.0)


def test_matrix_multiply_matches_numpy() -> None:
    a = [[1.0, 2.0], [3.0, 4.0]]
    b = [[5.0, 6.0], [7.0, 8.0]]
    ours = matrix_multiply(a, b)
    theirs = np.array(a) @ np.array(b)
    assert np.allclose(ours, theirs)


def test_matrix_multiply_identity() -> None:
    identity = [[1.0, 0.0], [0.0, 1.0]]
    v = [[3.0], [7.0]]
    result = matrix_multiply(identity, v)
    assert result.tolist() == [[3.0], [7.0]]


def test_l2_normalize() -> None:
    normalized = l2_normalize([3.0, 4.0])
    assert sum(c * c for c in normalized) == pytest.approx(1.0)


def test_l2_normalize_zero_vector() -> None:
    assert l2_normalize([0.0, 0.0]) == [0.0, 0.0]


def test_t_statistic_separable_samples() -> None:
    # Two clearly separated samples must produce a large |t|.
    t = t_statistic([1.0, 1.1, 0.9, 1.0], [50.0, 49.0, 51.0, 50.0])
    assert abs(t) > 10.0


def test_t_statistic_identical_samples() -> None:
    t = t_statistic([1.0, 2.0, 3.0], [1.0, 2.0, 3.0])
    assert t == pytest.approx(0.0)


def test_correlation_p_value_in_range() -> None:
    p = correlation_p_value(2.5, 10)
    assert 0.0 <= p <= 1.0


def test_dataframe_from_rows() -> None:
    frame = dataframe_from_rows([{"x": 1, "y": "a"}, {"x": 2, "y": "b"}])
    assert list(frame["x"]) == [1, 2]
    assert len(frame) == 2


def test_dataframe_from_rows_empty() -> None:
    assert len(dataframe_from_rows([])) == 0


def test_rows_of() -> None:
    frame = pd.DataFrame({"a": [1, 2], "b": [3, 4]})
    assert rows_of(frame) == [(1, 3), (2, 4)]


def test_polyfill_mean() -> None:
    assert polyfill_mean([1.0, 2.0, 3.0]) == 2.0


def test_polyfill_mean_empty_is_nan() -> None:
    # Guards the empty-input contract: polyfill_mean returns NaN, matching
    # numpy.mean's behavior on empty input.
    assert polyfill_mean([]) != polyfill_mean([])


def test_polyfill_abs() -> None:
    assert polyfill_abs(-3.0) == 3.0
    assert polyfill_abs(0.0) == 0.0


def test_polyfill_zip_roundtrip() -> None:
    # Verifies polyfill_zip matches built-in zip semantics.
    assert polyfill_zip([1, 2], ["a", "b"]) == list(zip([1, 2], ["a", "b"], strict=False))
