"""
Leetcode: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
"""

from utils import TestCase


def best_time_to_buy_and_sell_stock(prices: list[int]):
    min_price: int
    max_price: int

    min_price = max_price = prices[0]

    for price in prices:
        if price < min_price:
            min_price = price
            max_price = price
        if price > max_price:
            max_price = price

    return max_price - min_price


def best_time_to_buy_and_sell_stock_2(prices: list[int]) -> int:
    min_price = float("inf")
    max_profit = 0

    for price in prices:
        if price < min_price:
            min_price = price
        elif price - min_price > max_profit:
            max_profit = price - min_price

    return int(max_profit)


def test_best_time_to_buy_and_sell_stock():
    test_cases: list[TestCase] = [
        TestCase(input=[7, 1, 5, 3, 6, 4], expected=5),
        TestCase(input=[7, 6, 4, 3, 1], expected=0),
        TestCase(input=[10, 1, 5, 6, 7, 1], expected=6),
        TestCase(input=[10, 8, 7, 5, 2], expected=0),
    ]

    for case in test_cases:
        assert isinstance(case.input, list)
        assert best_time_to_buy_and_sell_stock(case.input) == case.expected
        assert best_time_to_buy_and_sell_stock_2(case.input) == case.expected
