// https://leetcode.com/problems/maximum-subarray/

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    // Let sum[i] be the best sum we can get when ending the subarray at index i
    //
    // The definition above means we must always take nums[i] into the subarray being considered.
    // Therefore, there are only 2 possibilities:
    // - The number can be merged with best subarray ending at i-1: sum[i] = sum[i-1] + nums[i]
    // - The number is negative and will decrease the subarray sum, so we should better start a new
    //   subarray to minimize the loss: sum[i] = nums[i]
    //
    // Note that sum[i] is for the subarray ending at i, but we don't know where the optimal subarray
    // ends, so we need to track for the best subarray sum separately
    //
    // Because sum[i] only requires sum[i-1], we don't really need to store all values, just the
    // latest sum is enough.
    let mut sum = 0;
    let mut best_sum = i32::MIN;
    for x in nums {
        sum = sum.max(0) + x;
        best_sum = best_sum.max(sum);
    }
    best_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }
    #[test]
    fn test_2() {
        assert_eq!(max_sub_array(vec![1]), 1);
    }
    #[test]
    fn test_3() {
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
    #[test]
    fn test_4() {
        assert_eq!(
            max_sub_array(vec![0, 0, 3, 3, 0, -2, 3, -2, -1, -1, 2, 1]),
            7
        );
    }
    #[test]
    fn test_5() {
        assert_eq!(max_sub_array(vec![0]), 0);
    }
    #[test]
    fn test_6() {
        assert_eq!(max_sub_array(vec![-2, 3, -3, 1, 1, -1, 1, 1, 1]), 4);
    }
    #[test]
    fn test_7() {
        assert_eq!(max_sub_array(vec![-1]), -1);
    }
}
