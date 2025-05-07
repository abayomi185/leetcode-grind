"""
Leetcode: https://leetcode.com/problems/problem_name/

# Use vim to replace all occurrences of problem_name with the actual problem name
s/problem_name/
"""

from utils import TestCase


def problem_name(nums, target):
    return nums + target


def test_problem_name():
    test_cases: list[TestCase] = [
        TestCase(input=[2, 7, 11, 15], target=9, expected=[0, 1]),
    ]

    for case in test_cases:
        assert problem_name(case.input, case.target) == case.expected
