"""
Leetcode: https://leetcode.com/problems/two-sum/
"""

from utils import TestCase


def two_sum(nums, target):
    num_to_index = {}
    for i, num in enumerate(nums):
        complement = target - num
        if complement in num_to_index:
            return [num_to_index[complement], i]
        num_to_index[num] = i
    return []


def test_two_sum():
    test_cases = [
        TestCase(input=([2, 7, 11, 15], 9), expected=[0, 1]),
        TestCase(input=([3, 2, 4], 6), expected=[1, 2]),
        TestCase(input=([3, 3], 6), expected=[0, 1]),
    ]

    for case in test_cases:
        assert case.input is not None
        assert two_sum(case.input[0], case.input[1]) == case.expected
