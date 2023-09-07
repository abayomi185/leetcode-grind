use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut num_map = HashMap::new();

        for num in nums {
            if num_map.contains_key(&num) {
                return true;
            }
            num_map.insert(num, 1);
        }
        false
    }
}
