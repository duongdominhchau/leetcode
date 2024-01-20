// https://leetcode.com/problems/merge-sorted-array

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i = m - 1;
    let mut j = n - 1;
    let mut k = (m + n - 1) as usize;
    while i >= 0 && j >= 0 {
        let index_i = i as usize;
        let index_j = j as usize;
        if nums1[index_i] > nums2[index_j] {
            nums1[k] = nums1[index_i];
            i -= 1;
        } else {
            nums1[k] = nums2[index_j];
            j -= 1;
        }
        k -= 1;
    }
    while i >= 0 {
        nums1[k] = nums1[i as usize];
        i -= 1;
        k = k.saturating_sub(1);
    }
    while j >= 0 {
        nums1[k] = nums2[j as usize];
        j -= 1;
        k = k.saturating_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut a = vec![1, 2, 3, 0, 0, 0];
        merge(&mut a, 3, &mut vec![2, 5, 6], 3);
        assert_eq!(a, vec![1, 2, 2, 3, 5, 6]);
    }
    #[test]
    fn test_2() {
        let mut a = vec![1];
        merge(&mut a, 1, &mut Vec::new(), 0);
        assert_eq!(a, vec![1]);
    }
    #[test]
    fn test_3() {
        let mut a = vec![0];
        merge(&mut a, 0, &mut vec![1], 1);
        assert_eq!(a, vec![1]);
    }
}
