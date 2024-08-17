from jellyfish import levenshtein_distance as levenshtein_jelly
from fast_levenshtein import (
    levenshtein_mat_py,
    levenshtein_vec_py,
    levenshtein_exp_py,
    levenshtein_cy,
    levenshtein_mat_rs,
    levenshtein_vec_rs,
    levenshtein_exp_rs,
)
from typing import Any, Dict, List

cases: List[Dict[str, Any]] = [
    {"a": "flaw", "b": "lawn", "expected": 2},
    {"a": "intention", "b": "execution", "expected": 5},
    {"a": "giraffe", "b": "gorilla", "expected": 5},
    {"a": "book", "b": "back", "expected": 2},
    {"a": "apple", "b": "apricot", "expected": 5},
    {"a": "hello", "b": "hallo", "expected": 1},
    {"a": "algorithm", "b": "altruistic", "expected": 6},
    {"a": "abcdefg", "b": "abcdxyz", "expected": 3},
    {"a": "mouse", "b": "mouses", "expected": 1},
    {"a": "sunday", "b": "saturday", "expected": 3},
]


def test_levenshtein_mat_rs() -> None:
    for case in cases:
        assert levenshtein_mat_rs(case["a"], case["b"]) == case["expected"]


def test_levenshtein_vec_rs() -> None:
    for case in cases:
        assert levenshtein_vec_rs(case["a"], case["b"]) == case["expected"]


def test_levensthein_exp_rs() -> None:
    for case in cases:
        assert levenshtein_exp_rs(case["a"], case["b"]) == case["expected"]


def test_levenshtein_jelly() -> None:
    for case in cases:
        assert levenshtein_jelly(case["a"], case["b"]) == case["expected"]


def test_levenshtein_mat_py() -> None:
    for case in cases:
        assert levenshtein_mat_py(case["a"], case["b"]) == case["expected"]


def test_levenshtein_vec_py() -> None:
    for case in cases:
        assert levenshtein_vec_py(case["a"], case["b"]) == case["expected"]


def test_levenshtein_exp_py() -> None:
    for case in cases:
        assert levenshtein_exp_py(case["a"], case["b"]) == case["expected"]


def test_levenshtein_cy() -> None:
    for case in cases:
        assert levenshtein_cy(case["a"], case["b"]) == case["expected"]
