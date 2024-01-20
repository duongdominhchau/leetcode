// https://leetcode.com/problems/sum-of-two-integers

pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let carry = a & b;
        a ^= b;
        b = carry << 1;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(get_sum(1, 2), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(get_sum(2, 3), 5);
    }
}
