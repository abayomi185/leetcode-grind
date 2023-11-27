#[cfg(test)]
use crate::helpers::tree::{create_tree_from_vec, TreeNode};
#[cfg(test)]
use std::{cell::RefCell, rc::Rc};

#[cfg(test)]
pub fn invert_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    // Rc is a reference-counted pointer
    // RefCell is a type of cell providing interior mutability

    // Recursion is the answer
    // Rc allows multiple ownership of the same data
    // which allows the clone of the root node
    if let Some(node) = root.clone() {
        // borrow_mut() returns a mutable reference to the wrapped value
        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();
        // Swap the left and right child nodes
        node.borrow_mut().left = invert_binary_tree(right);
        node.borrow_mut().right = invert_binary_tree(left);
    }

    root
}

#[test]
fn test_invert_binary_tree() {
    let test_cases: [(Vec<i32>, Vec<i32>); 3] = [
        (vec![4, 2, 7, 1, 3, 6, 9], vec![4, 7, 2, 9, 6, 3, 1]),
        (vec![2, 1, 3], vec![2, 3, 1]),
        (vec![], vec![]),
    ];

    for (input, expected) in test_cases {
        let tree_root = create_tree_from_vec(input);
        let expected_tree = create_tree_from_vec(expected);

        assert_eq!(invert_binary_tree(tree_root), expected_tree);
    }
}
