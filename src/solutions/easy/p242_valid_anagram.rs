/*
Progression
1. Started with 2 loops and 1 HashMap

2. Changed to 1 loop with 2 HashMaps

3. Changed to 1 loop with 1 HashMap incrementing and decrementing values. Then checking that all
are 0
*/
#[cfg(test)]
fn is_anagram(s: String, t: String) -> bool {
    use std::collections::HashMap;

    if s.len() != t.len() {
        return false;
    }
    let mut char_map = HashMap::new();
    for (char_s, char_t) in s.chars().zip(t.chars()) {
        *char_map.entry(char_s).or_insert(0) += 1;
        *char_map.entry(char_t).or_insert(0) -= 1;
    }
    !char_map.into_values().any(|count| count != 0)
}

#[test]
fn test() {
    let test_cases: [((&str, &str), bool); 2] =
        [(("anagram", "nagaram"), true), (("rat", "car"), false)];

    for (case, want) in test_cases {
        assert_eq!(is_anagram(case.0.to_string(), case.1.to_string()), want);
    }
}
