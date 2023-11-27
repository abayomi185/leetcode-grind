#[cfg(test)]
use std::collections::VecDeque;
use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[cfg(test)]
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
// Helper function to convert a tree to a vector
pub fn tree_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    if let Some(node) = root {
        let mut queue = VecDeque::new();
        queue.push_back(node.clone());
        while let Some(current) = queue.pop_front() {
            let current = current.borrow();
            result.push(current.val);
            if let Some(left) = &current.left {
                queue.push_back(left.clone());
            }
            if let Some(right) = &current.right {
                queue.push_back(right.clone());
            }
        }
    }
    result
}

#[cfg(test)]
// Helper function to create a tree from a vector
pub fn create_tree_from_vec(vec: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if vec.is_empty() {
        return None;
    }
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0]))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());

    for chunk in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(&v) = chunk.first() {
            let left = Rc::new(RefCell::new(TreeNode::new(v)));
            parent.borrow_mut().left = Some(left.clone());
            queue.push_back(left);
        }
        if let Some(&v) = chunk.get(1) {
            let right = Rc::new(RefCell::new(TreeNode::new(v)));
            parent.borrow_mut().right = Some(right.clone());
            queue.push_back(right);
        }
    }

    head
}
