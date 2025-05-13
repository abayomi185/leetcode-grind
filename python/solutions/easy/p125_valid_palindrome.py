"""
Leetcode: https://leetcode.com/problems/valid-palindrome/
"""

import re

from utils import TestCase


def valid_palindrome(input: str):
    string = re.sub(r"\W+", "", input.lower())
    reverse_string = string[::-1]

    for pointer in range(0, len(string)):
        if string[pointer] != reverse_string[pointer]:
            return False

    return True


def test_valid_palindrome():
    test_cases: list[TestCase] = [
        TestCase(input="Was it a car or a cat I saw?", expected=True),
        TestCase(input="tab a cat", expected=False),
        TestCase(input="A man, a plan, a canal: Panama", expected=True),
        TestCase(input="race a car", expected=False),
        TestCase(input=" ", expected=True),
    ]

    for case in test_cases:
        assert valid_palindrome(str(case.input)) == case.expected
