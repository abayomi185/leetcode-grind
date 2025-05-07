"""
Leetcode: https://leetcode.com/problems/contains-duplicate/
"""

from utils import TestCase


def contains_duplicate(nums):
    original_length = len(nums)
    length_of_set = len(set(nums))

    return original_length != length_of_set


def test_contains_duplicate():
    test_cases: list[TestCase] = [
        TestCase(input=[1, 2, 3, 1], expected=True),
        TestCase(input=[1, 2, 3, 4], expected=False),
        TestCase(input=[1, 1, 1, 3, 3, 4, 3, 2, 4, 2], expected=True),
    ]

    for case in test_cases:
        assert contains_duplicate(case.input) == case.expected
