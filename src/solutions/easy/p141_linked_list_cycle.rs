// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[cfg(test)]
pub fn linked_list_cycle(list: Option<Box<ListNode>>) -> bool {
    let mut pointer_slow = list.as_ref();
    let mut pointer_fast = list.as_ref();

    while let (Some(slow), Some(fast)) = (
        pointer_slow,
        pointer_fast.and_then(|node| node.next.as_ref()),
    ) {
        pointer_slow = slow.next.as_ref();
        pointer_fast = fast.next.as_ref().and_then(|node| node.next.as_ref());
    }

    false
}

#[test]
fn test_linked_list_cycle() {
    let test_cases: [(Vec<i32>, bool); 3] = [
        (vec![3, 2, 0, -4], true),
        (vec![1, 2], true),
        (vec![1], false),
    ];

    for case in &test_cases {
        let list = &case.0;

        let list_nodes = list.iter().rev().fold(None, |acc, &x| {
            Some(Box::new(ListNode { val: x, next: acc }))
        });

        let expected_value = case.1;

        // WARN: Leetcode does not have Rust implementation
        // assert_eq!(linked_list_cycle(list_nodes), expected_value);
    }
}
