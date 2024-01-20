// https://leetcode.com/problems/find-polygon-with-the-largest-perimeter

pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
    let n = nums.len();
    nums.sort_unstable();

    let mut sum = vec![0i64; n];
    sum[0] = nums[0] as i64;
    for i in 1..n {
        sum[i] = sum[i - 1] + nums[i] as i64;
    }

    for i in (2..n).rev() {
        if sum[i - 1] > nums[i] as i64 {
            return sum[i];
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(largest_perimeter(vec![5, 5, 5]), 15);
    }
    #[test]
    fn test_2() {
        assert_eq!(largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]), 12);
    }
    #[test]
    fn test_3() {
        assert_eq!(largest_perimeter(vec![5, 5, 50]), -1);
    }
}
