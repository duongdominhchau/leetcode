// https://leetcode.com/problems/same-tree

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() {
        return q.is_none();
    }
    if q.is_none() {
        return p.is_none();
    }
    let (p, q) = (p.unwrap(), q.unwrap());
    let (p, q) = (p.borrow(), q.borrow());
    p.val == q.val
        && is_same_tree(p.left.clone(), q.left.clone())
        && is_same_tree(p.right.clone(), q.right.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let p1 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let p2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let mut p_node = TreeNode::new(1);
        p_node.left = p1;
        p_node.right = p2;
        let p = Some(Rc::new(RefCell::new(p_node)));
        let q1 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let mut q_node = TreeNode::new(1);
        q_node.left = q1;
        q_node.right = q2;
        let q = Some(Rc::new(RefCell::new(q_node)));
        assert_eq!(is_same_tree(p, q), true);
    }
}
