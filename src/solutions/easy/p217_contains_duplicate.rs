/*
Leetcode: https://leetcode.com/problems/contains-duplicate/
*/
#[cfg(test)]
fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;

    let mut num_set = HashSet::new();
    for num in nums {
        if num_set.contains(&num) {
            return true;
        }
        num_set.insert(num);
    }
    false
}

#[test]
fn test_contains_duplicate() {
    let test_cases: [(Vec<i32>, bool); 3] = [
        (vec![1, 2, 3, 1], true),
        (vec![1, 2, 3, 4], false),
        (vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true),
    ];

    for (case, want) in test_cases {
        assert_eq!(contains_duplicate(case), want);
    }
}
