// https://leetcode.com/problems/number-of-1-bits/

pub fn hamming_weight(n: u32) -> i32 {
    (0..32).map(|k| if n & (1 << k) != 0 { 1 } else { 0 }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(hamming_weight(0b00000000000000000000000000001011), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(hamming_weight(0b00000000000000000000000010000000), 1);
    }
    #[test]
    fn test_3() {
        assert_eq!(hamming_weight(0b11111111111111111111111111111101), 31);
    }
}
