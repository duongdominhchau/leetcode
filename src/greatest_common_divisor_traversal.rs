// https://leetcode.com/problems/greatest-common-divisor-traversal

struct DisjointSet {
    parent: Option<usize>,
}

impl DisjointSet {
    pub fn new() -> Self {
        Self { parent: None }
    }
}

pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_1() {
    //     assert_eq!(can_traverse_all_pairs(vec![2, 3, 6]), true);
    // }
    // #[test]
    // fn test_2() {
    //     assert_eq!(can_traverse_all_pairs(vec![3, 9, 5]), false);
    // }
    // #[test]
    // fn test_3() {
    //     assert_eq!(can_traverse_all_pairs(vec![4, 3, 12, 8]), true);
    // }
    // #[test]
    // fn test_4() {
    //     assert_eq!(can_traverse_all_pairs(vec![1]), true);
    // }
}
