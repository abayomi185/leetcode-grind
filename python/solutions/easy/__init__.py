from solutions.easy.p1_two_sum import test_two_sum
from solutions.easy.p20_valid_parenthesis import test_valid_parenthesis
from solutions.easy.p121_best_time_to_buy_and_sell_stock import (
    test_best_time_to_buy_and_sell_stock,
)
from solutions.easy.p125_valid_palindrome import test_valid_palindrome
from solutions.easy.p217_contains_duplicate import test_contains_duplicate
from solutions.easy.p242_valid_anagram import test_valid_anagram
from solutions.easy.p704_binary_search import test_binary_search


def test_all_easy():
    test_binary_search()  # P704
    test_best_time_to_buy_and_sell_stock()  # P121
    test_contains_duplicate()  # P217
    test_two_sum()  # P2
    test_valid_anagram()  # P242
    test_valid_palindrome()  # P125
    test_valid_parenthesis()  # P20
