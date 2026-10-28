"""Tests for the ML layer. The calibration protocol is exercised in full."""

import math

import pytest

from outputplusplus.ml import (
    accuracy,
    feature_scale,
    fit_calibrated_knn,
    predict_with_confidence,
    train_test_split,
)


def _separable_data() -> tuple[list[list[float]], list[int]]:
    X = [[0.0, 0.0], [0.1, 0.0], [0.0, 0.1], [5.0, 5.0], [5.1, 5.0], [5.0, 5.1]]
    y = [0, 0, 0, 1, 1, 1]
    return X, y


def test_train_test_split_partitions() -> None:
    X, y = _separable_data()
    split = train_test_split(X, y, test_size=0.5)
    assert len(split["X_train"]) + len(split["X_test"]) == len(X)
    assert len(split["y_train"]) + len(split["y_test"]) == len(y)
    # Deterministic: same input, same split, forever.
    again = train_test_split(X, y, test_size=0.5)
    assert split["X_train"] == again["X_train"]


def test_train_test_split_no_overlap() -> None:
    X, y = _separable_data()
    split = train_test_split(X, y, test_size=0.5)
    train_pairs = {tuple(r) for r in split["X_train"]}
    test_pairs = {tuple(r) for r in split["X_test"]}
    assert not (train_pairs & test_pairs)


def test_fit_calibrated_knn_predicts_separable() -> None:
    X, y = _separable_data()
    model = fit_calibrated_knn(X, y, k=1)
    predictions = model.predict([[0.0, 0.0], [5.0, 5.0]])
    assert list(predictions) == [0, 1]


def test_predict_with_confidence() -> None:
    X, y = _separable_data()
    model = fit_calibrated_knn(X, y, k=1)
    results = predict_with_confidence(model, [[0.0, 0.0], [5.0, 5.0]])
    assert [r[0] for r in results] == [0, 1]
    for _, confidence in results:
        assert 0.0 < confidence <= 1.0
        assert not math.isnan(confidence)


def test_feature_scale_standardizes() -> None:
    X = [[1.0, 10.0], [3.0, 10.0], [5.0, 10.0]]
    scaled = feature_scale(X)
    # Column 0 is standardized to zero mean and unit variance; column 1
    # is constant and maps to 0.0. (1-3)/sqrt(8/3) = -sqrt(3/2).
    assert scaled[0][0] == pytest.approx(-(3.0 / 2.0) ** 0.5)
    assert scaled[1][0] == pytest.approx(0.0)
    assert scaled[2][0] == pytest.approx((3.0 / 2.0) ** 0.5)
    assert scaled[0][1] == 0.0
    assert scaled[2][1] == 0.0


def test_accuracy() -> None:
    assert accuracy([1, 2, 3], [1, 2, 3]) == 1.0
    assert accuracy([1, 2, 3], [1, 2, 4]) == pytest.approx(2 / 3)
    assert accuracy([], []) == 0.0
