// https://leetcode.com/problems/divide-array-into-arrays-with-max-difference

pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    // If a number can match with either `a` or `b` where `a < b`, then matching it with `a` is
    // always better because if we have a larger number `c`, `c - b` will be smaller than `c - a`
    // and thus is more likely to be able to satisfy the condition distance <= k
    nums.sort_unstable();
    let mut result = Vec::new();
    for chunk in nums.chunks(3) {
        if chunk[2] - chunk[0] > k {
            return Vec::new();
        }
        result.push(chunk.to_vec());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2),
            vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            divide_array(vec![1, 3, 3, 2, 7, 3], 3),
            Vec::<Vec::<i32>>::new()
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            divide_array(
                vec![15, 13, 12, 13, 12, 14, 12, 2, 3, 13, 12, 14, 14, 13, 5, 12, 12, 2, 13, 2, 2],
                2
            ),
            Vec::<Vec::<i32>>::new()
        );
    }
}
