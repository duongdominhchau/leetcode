// https://leetcode.com/problems/house-robber-ii

fn rob1(nums: &[i32]) -> i32 {
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
pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 2 {
        return nums[0];
    }
    // Idea: We consider two situations:
    // - The first house is robbed: In this case, we must also skip the last house, so the range to
    // consider is restricted to 0..(n-1)
    // - The first house is not robbed, then we can rob the last house => The range is 1..n
    rob1(&nums[0..n - 1]).max(rob1(&nums[1..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(rob(vec![2, 3, 2]), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }
    #[test]
    fn test_3() {
        assert_eq!(rob(vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_4() {
        assert_eq!(rob(vec![0]), 0);
    }
}
