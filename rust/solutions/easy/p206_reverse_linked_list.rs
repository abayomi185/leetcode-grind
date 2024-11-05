#[cfg(test)]
use crate::helpers::linked_list::ListNode;

#[cfg(test)]
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut prev = None;
    while let Some(mut boxed_node) = head {
        head = boxed_node.next.take();
        boxed_node.next = prev;
        prev = Some(boxed_node);
    }
    prev

    // A fold may also be used
}

#[test]
fn test_reverse_list() {
    let test_cases: [Vec<i32>; 3] = [(vec![1, 2, 3, 4, 5]), (vec![1, 2]), (vec![])];

    for case in test_cases {
        let mut head = None;
        let mut rev_head = None;
        for (val_forward, val_backward) in case.iter().zip(case.iter().rev()) {
            let mut node_backward = ListNode::new(*val_forward);
            node_backward.next = rev_head;
            rev_head = Some(Box::new(node_backward));

            let mut node_forward = ListNode::new(*val_backward);
            node_forward.next = head;
            head = Some(Box::new(node_forward));
        }
        assert_eq!(reverse_list(head), rev_head);
    }
}
