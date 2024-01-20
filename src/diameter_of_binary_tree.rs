// https://leetcode.com/problems/diameter-of-binary-tree

use std::cell::RefCell;
use std::rc::Rc;

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

fn traverse(root: Option<Rc<RefCell<TreeNode>>>, best: &mut i32) -> i32 {
    match root {
        Some(node) => {
            let node = node.borrow();
            // Longest path going past `root` is the path from the deepest node to the left
            // to the deepest node to the right
            //
            // Current node may not be the optimal solution. Consider a node with only
            // right child of depth 5. If we stop at that node, we get 5 as the best answer.
            // However, if the right child has a subtree with depth 4 as its left child and
            // another subtree of depth 3 as its right child, the longest path can be 8.
            let left = traverse(node.left.clone(), best);
            let right = traverse(node.right.clone(), best);
            let current = left + right;
            if current > *best {
                *best = current;
            }
            // If this node is the answer, we should have the path length recorded in `best`.
            // Here we only care about the case we don't choose current node as center.
            // In that case, we will only be able to visit one of two branches here, hence
            // the max() call
            left.max(right) + 1
        }
        None => 0,
    }
}
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut best = 0;
    traverse(root, &mut best);
    best
}
