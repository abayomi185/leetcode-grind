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
pub fn merge_two_sorted_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list1 = list1;
    let mut list2 = list2;

    let mut merged_list: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
    let mut tail = &mut merged_list;

    // Walk through both lists at the same time, comparing the values of each node.
    // Walk to the next node where the value is smaller.
    while list1.is_some() || list2.is_some() {
        match (list1.as_ref(), list2.as_ref()) {
            (Some(node1), Some(node2)) if node1.val <= node2.val => {
                tail.as_mut().unwrap().next = list1;
                tail = &mut tail.as_mut().unwrap().next;
                list1 = tail.as_mut().unwrap().next.take()
            }
            (Some(_), Some(_)) => {
                tail.as_mut().unwrap().next = list2;
                tail = &mut tail.as_mut().unwrap().next;
                list2 = tail.as_mut().unwrap().next.take()
            }
            (Some(_), None) => {
                tail.as_mut().unwrap().next = list1.take();
                break;
            }
            (None, Some(_)) => {
                tail.as_mut().unwrap().next = list2.take();
                break;
            }
            (None, None) => break,
        }
    }

    merged_list.unwrap().next
}

#[test]
fn test_merge_two_sorted_lists() {
    type MergeTwoListsInputType = (Vec<i32>, Vec<i32>);
    let test_cases: [(MergeTwoListsInputType, Vec<i32>); 3] = [
        ((vec![1, 2, 4], vec![1, 3, 4]), vec![1, 1, 2, 3, 4, 4]),
        ((vec![], vec![]), vec![]),
        ((vec![], vec![0]), vec![0]),
    ];

    for case in &test_cases {
        let (list1, list2) = &case.0;

        let list1_nodes = list1.iter().rev().fold(None, |acc, &x| {
            Some(Box::new(ListNode { val: x, next: acc }))
        });
        let list2_nodes = list2.iter().rev().fold(None, |acc, &x| {
            Some(Box::new(ListNode { val: x, next: acc }))
        });

        let expected = &case.1;
        let expected_node = expected.iter().rev().fold(None, |acc, &x| {
            Some(Box::new(ListNode { val: x, next: acc }))
        });

        assert_eq!(
            merge_two_sorted_lists(list1_nodes, list2_nodes),
            expected_node
        );
    }
}
