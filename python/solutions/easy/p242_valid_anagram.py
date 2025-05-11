"""
Leetcode: https://leetcode.com/problems/valid-anagram/
"""

from utils import TestCase


def valid_anagram(nums, target):
    return nums + target


def test_valid_anagram():
    test_cases: list[TestCase] = [
        TestCase(input=[2, 7, 11, 15], target=9, expected=[0, 1]),
    ]

    for case in test_cases:
        assert valid_anagram(case.input, case.target) == case.expected
