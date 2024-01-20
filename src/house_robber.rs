// https://leetcode.com/problems/house-robber

pub fn rob_dp(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 2 {
        return nums[0];
    }

    // best[i] = max amount achievable if we take nums[i]
    let mut best = vec![0; n];
    best[0] = nums[0];
    best[1] = best[0].max(nums[1]);
    for i in 2..n {
        best[i] = best[i - 1].max(best[i - 2] + nums[i]);
    }
    best[n - 1]
}
pub fn rob_dp_space_optimized(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums[0];
    }

    let mut max_if_skipped_prev = 0;
    let mut max_if_taken_prev = nums[0];
    for i in 1..nums.len() {
        // We can only take current value if previous element is skipped
        let current_max = max_if_taken_prev.max(max_if_skipped_prev + nums[i]);
        // Calculate state for the next position
        // If we took previous element, we can't take current element, so when we consider the next
        // iteration, this is the max value when skipped previous element
        max_if_skipped_prev = max_if_taken_prev;
        // Similarly, if we take current element, the max obtained will be max_if_taken_prev for
        // the next iteration
        max_if_taken_prev = current_max;
    }
    max_if_taken_prev
}
pub fn rob(nums: Vec<i32>) -> i32 {
    rob_dp(nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }
    #[test]
    fn test_2() {
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    }
    #[test]
    fn test_3() {
        assert_eq!(rob(vec![1, 2]), 2);
    }
    #[test]
    fn test_4() {
        assert_eq!(rob(vec![2, 1, 1, 2]), 4);
    }
}
