from jellyfish import levenshtein_distance as levenshtein_jelly
from fxkk import levenshtein
from rapidfuzz.distance import Levenshtein

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
    {"a": "дом", "b": "том", "expected": 1},
    {"a": "привет", "b": "пока", "expected": 5},
    {"a": "молоко", "b": "молоток", "expected": 2},
    {"a": "стол", "b": "стул", "expected": 1},
    {"a": "кот", "b": "кит", "expected": 1},
    {"a": "работа", "b": "работник", "expected": 3},
    {"a": "осень", "b": "весна", "expected": 4},
    {"a": "собака", "b": "кошка", "expected": 3},
    {"a": "мир", "b": "мирный", "expected": 3},
]


def test_levensthein_tweaked_rs() -> None:
    for case in cases:
        assert levenshtein(case["a"], case["b"]) == case["expected"]


def test_levenshtein_jelly() -> None:
    for case in cases:
        assert levenshtein_jelly(case["a"], case["b"]) == case["expected"]


def test_rapidfuzz() -> None:
    for case in cases:
        assert Levenshtein.distance(case["a"], case["b"]) == case["expected"]
