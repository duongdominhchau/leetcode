// https://leetcode.com/problems/intersection-of-two-arrays

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;

    let s1: HashSet<i32> = nums1.into_iter().collect();
    let s2: HashSet<i32> = nums2.into_iter().collect();
    s1.intersection(&s2).into_iter().map(|&x| x).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: Vec<i32>, b: Vec<i32>, mut expected: Vec<i32>) {
        expected.sort_unstable();
        let mut actual = intersection(a, b);
        actual.sort_unstable();
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_1() {
        check(vec![1, 2, 2, 1], vec![2, 2], vec![2]);
    }
    #[test]
    fn test_2() {
        check(vec![4, 9, 5], vec![9, 4, 9, 8, 4], vec![4, 9]);
    }
}
