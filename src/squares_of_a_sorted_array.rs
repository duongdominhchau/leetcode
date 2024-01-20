// https://leetcode.com/problems/squares-of-a-sorted-array/

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();

    let mut left = 0;
    let mut right = n - 1;

    let mut result = vec![0; n];
    for k in (0..n).rev() {
        let sqr_left = nums[left].pow(2);
        let sqr_right = nums[right].pow(2);
        if sqr_left > sqr_right {
            result[k] = sqr_left;
            left += 1;
        } else {
            result[k] = sqr_right;
            right = right.saturating_sub(1);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(sorted_squares(vec![1]), vec![1]);
    }
}
