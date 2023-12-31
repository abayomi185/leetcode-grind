/*
Leetcode: https://leetcode.com/problems/valid-palindrome/

Progression
1. Started with 2 separate pointers for left and right. Doing alphanumeric checks first,
then comparing the left and right char

2. Functional way filtering out non-alphanumeric characters and making sure to lowercase all characters (missed this many times)
the comparing the reverse of the vec, adhering to Rust borrow checking rules
*/
#[cfg(test)]
fn is_palindrome(s: String) -> bool {
    // let chars: Vec<char> = s.chars().flat_map(|x| x.to_lowercase()).collect();

    // let mut left_pointer = 0;
    // let mut right_pointer = chars.len() - 1;

    // while left_pointer < right_pointer {
    //     if !chars[left_pointer].is_alphanumeric() {
    //         left_pointer += 1;
    //         continue;
    //     }
    //     if !chars[right_pointer].is_alphanumeric() {
    //         right_pointer -= 1;
    //         continue;
    //     }
    //     if chars[left_pointer] != chars[right_pointer] {
    //         return false;
    //     }
    //     left_pointer += 1;
    //     right_pointer -= 1;
    // }
    // true

    let iter = s
        .chars()
        .filter(|x| x.is_ascii_alphanumeric())
        .map(|x| x.to_ascii_lowercase());

    iter.clone().eq(iter.rev())
}

#[test]
fn test_valid_palindrome() {
    let test_cases: [(String, bool); 3] = [
        (String::from("A man, a plan, a canal: Panama"), true),
        (String::from("race a car"), false),
        (String::from(" "), true),
    ];

    for (case, want) in test_cases {
        assert_eq!(is_palindrome(case), want);
    }
}
