// https://leetcode.com/problems/rearrange-array-elements-by-sign

pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![0; n];
    let mut indices = vec![0; 2];
    for i in 0..nums.len() {
        let index = &mut indices[i % 2];
        let sign = if i % 2 == 0 { 1 } else { -1 };
        while *index < n && nums[*index].signum() != sign {
            *index += 1;
        }
        result[i] = nums[*index];
        *index += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            rearrange_array(vec![3, 1, -2, -5, 2, -4]),
            vec![3, -2, 1, -5, 2, -4]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(rearrange_array(vec![-1, 1]), vec![1, -1]);
    }
}
