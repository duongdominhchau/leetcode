// https://leetcode.com/problems/plus-one

pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 1;
    for i in (0..digits.len()).rev() {
        let sum = digits[i] + carry;
        digits[i] = sum % 10;
        carry = sum / 10;
    }
    if carry > 0 {
        digits.insert(0, carry);
    }
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }
    #[test]
    fn test_2() {
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }
    #[test]
    fn test_3() {
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
    }
}
