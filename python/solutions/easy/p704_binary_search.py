"""
Leetcode: https://leetcode.com/problems/binary-search/
"""

from math import floor

from utils import TestCase


def binary_search(nums, target):

    midpoint = lambda x: floor(x / 2)

    def recursive_block(slice):
        mid = midpoint(len(slice)) + 1
        value = nums[mid]
        if value == target:
            return mid
        elif target > value:
            return recursive_block(nums[mid:-1])
        else:
            return recursive_block(nums[0:mid])

    return recursive_block(nums)


def test_binary_search():
    test_cases: list[TestCase] = [
        TestCase(input=([-1, 0, 3, 5, 9, 12], 9), expected=4),
        TestCase(input=([-1, 0, 3, 5, 9, 12], 2), expected=-1),
        TestCase(input=([-1, 0, 2, 4, 6, 8], 4), expected=3),
        TestCase(input=([-1, 0, 2, 4, 6, 8], 3), expected=-1),
    ]

    for case in test_cases:
        assert case.input is not None
        assert binary_search(case.input[0], case.input[1]) == case.expected
