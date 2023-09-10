/*
Leetcode: https://leetcode.com/problems/valid-parenthesis/
*/
#[cfg(test)]
fn is_valid(s: String) -> bool {
    // use std::collections::HashMap;
    // let parenthesis_map = HashMap::from([(')', '('), (']', '['), ('}', '{')]);

    let mut stack: Vec<char> = vec![];

    for char in s.chars() {
        // match parenthesis_map.get(&char) {
        //     Some(&last_open_bracket) => {
        //         if stack.pop() != Some(last_open_bracket) {
        //         }
        //             return false;
        //     }
        //     None => stack.push(char),
        // }
        match char {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '}' | ')' | ']' => {
                if stack.pop() != Some(char) {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}

#[test]
fn test_is_valid() {
    let test_cases: [(String, bool); 4] = [
        (String::from("()"), true),
        (String::from("()[]{}"), true),
        (String::from("(]"), false),
        (String::from("{[]}"), true),
    ];

    for (case, want) in test_cases {
        assert_eq!(is_valid(case), want);
    }
}
