// https://leetcode.com/problems/longest-increasing-subsequence

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut len = vec![0; n];
    len[0] = 1;
    for i in 1..n {
        let prev_best = (0..i)
            .filter(|&j| nums[j] < nums[i])
            .map(|j| len[j])
            .max()
            .unwrap_or(0);
        len[i] = prev_best + 1;
    }
    len.into_iter().max().unwrap() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
    #[test]
    fn test_2() {
        assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }
    #[test]
    fn test_3() {
        assert_eq!(length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
    #[test]
    fn test_4() {
        assert_eq!(length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]), 6);
    }
}
