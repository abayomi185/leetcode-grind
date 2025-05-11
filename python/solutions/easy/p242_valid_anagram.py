"""
Leetcode: https://leetcode.com/problems/valid-anagram/
"""

from typing import Counter

from utils import TestCase


def valid_anagram(s: str, t: str):
    if len(s) != len(t):
        return False

    s_character_map = {}
    t_character_map = {}

    for s_char, t_char in zip(s, t):
        s_character_map.setdefault(s_char, 0)
        t_character_map.setdefault(t_char, 0)

        s_character_map[s_char] += 1
        t_character_map[t_char] += 1

    for s_key in s_character_map.keys():
        if not t_character_map.get(s_key):
            return False

        if s_character_map[s_key] != t_character_map[s_key]:
            return False

    return True


def valid_anagram_2(s: str, t: str):
    return Counter(s) == Counter(t)


def test_valid_anagram():
    test_cases: list[TestCase] = [
        TestCase(input="racecar", target="carrace", expected=True),
        TestCase(input="jar", target="jam", expected=False),
        TestCase(input="aa", target="bb", expected=False),
    ]

    for case in test_cases:
        assert valid_anagram(str(case.input), str(case.target)) == case.expected
        assert valid_anagram_2(str(case.input), str(case.target)) == case.expected
