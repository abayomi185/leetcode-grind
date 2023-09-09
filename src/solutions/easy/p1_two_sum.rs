/*
Leetcode: https://leetcode.com/problems/two-sum/

This implementation is okay

Improvements
1. Another thing that could be done is sorting the nums array first then using two pointers, one
from the left and the other from the right
*/
#[cfg(test)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    // let mut num_map: HashMap<i32, i32> = HashMap::new();

    // To keep memory use in check, initialise HashMap with fixed capacity
    let mut num_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

    for (index, num) in nums.iter().enumerate() {
        // Using if-else, not the Rust way
        // if num_map.contains_key(num) {
        //     return vec![*num_map.get(num).unwrap(), index as i32];
        // } else {
        //     num_map.insert(target - num, index as i32);
        // }

        // The Rust way - match statement
        match num_map.contains_key(num) {
            true => return vec![*num_map.get(num).unwrap(), index as i32],
            false => num_map.insert(target - num, index as i32),
        };
    }
    vec![]
}

#[test]
fn test_two_sum() {
    type TwoSumInputType = (Vec<i32>, i32);
    let test_cases: [(TwoSumInputType, Vec<i32>); 3] = [
        ((vec![2, 7, 11, 15], 9), vec![0, 1]),
        ((vec![3, 2, 4], 6), vec![1, 2]),
        ((vec![3, 3], 6), vec![0, 1]),
    ];

    for (case, want) in test_cases {
        assert_eq!(two_sum(case.0, case.1), want);
    }
}
