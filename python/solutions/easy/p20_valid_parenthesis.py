"""
Leetcode: https://leetcode.com/problems/valid-parenthesis/
"""

from utils import TestCase


def valid_parenthesis(s):
    pairs = {
        ")": "(",
        "]": "[",
        "}": "{",
    }
    stack = []

    for char in s:
        if stack and char in pairs.keys():
            if stack[-1] == pairs[char]:
                stack.pop()
        else:
            stack.append(char)

    return not stack


def test_valid_parenthesis():
    test_cases: list[TestCase] = [
        TestCase(input="()", expected=True),
        TestCase(input="()[]{}", expected=True),
        TestCase(input="(]", expected=False),
        TestCase(input="([])", expected=True),
        TestCase(input="[]", expected=True),
        TestCase(input="([{}])", expected=True),
        TestCase(input="[(])", expected=False),
    ]

    for case in test_cases:
        assert valid_parenthesis(case.input) == case.expected
