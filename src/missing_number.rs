// https://leetcode.com/problems/missing-number/description/

pub fn missing_number(nums: Vec<i32>) -> i32 {
    // a XOR a == 0. nums has one number missing while [0; n] has all numbers. XOR will cancel out
    // all the numbers, leaving only the missing one there
    let correct_xor = (1..=nums.len()).fold(0, |acc, current| acc ^ current);
    let actual_xor = nums.into_iter().fold(0, |acc, current| acc ^ current);
    correct_xor as i32 ^ actual_xor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
    }
    #[test]
    fn test_2() {
        assert_eq!(missing_number(vec![0, 1]), 2);
    }
    #[test]
    fn test_3() {
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
