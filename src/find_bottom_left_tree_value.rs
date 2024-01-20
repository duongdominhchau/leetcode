// https://leetcode.com/problems/find-bottom-left-tree-value/

// pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//     use std::collections::VecDeque;

//     let mut prev_val = -1;
//     let mut prev_depth = -1;
//     let mut queue = VecDeque::new();
//     queue.push_back((root, 0, true));
//     while let (node, depth, is_left) = queue.pop_back() {
//         if is_left && depth > prev_depth {
//             prev_depth = depth;
//             prev_val = node;
//         }
//         queue.push_back(node);
//         best = 0;
//     }
// }
