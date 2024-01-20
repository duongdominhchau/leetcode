// https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list

// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

// pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     if head.is_none() || head.as_ref().unwrap().next.is_none() {
//         return head;
//     }
//     // From this point on, the list has at least 2 nodes
//     // Idea: sum[i] = sum up to i
//     // Sum from i to j = sum[i] - sum[j] + a[j]
//     let mut sum = vec![head.as_ref().unwrap().val];
//     let mut len = 1;
//     // TODO: Iterate until p reached the end
//     let mut p = head.as_ref().unwrap().next;
//     None
// }
