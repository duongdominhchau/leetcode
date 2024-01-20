// https://leetcode.com/problems/minimum-common-value/

pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    use std::cmp::Ordering;

    let mut left = 0;
    let mut right = 0;
    while left < nums1.len() && right < nums2.len() {
        match nums1[left].cmp(&nums2[right]) {
            Ordering::Less => left += 1,
            Ordering::Equal => return nums1[left] as i32,
            Ordering::Greater => right += 1,
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(get_common(vec![1, 2, 3], vec![2, 4]), 2);
    }
    #[test]
    fn test_2() {
        assert_eq!(get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]), 2);
    }
}
