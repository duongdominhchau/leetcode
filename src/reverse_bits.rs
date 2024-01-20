// https://leetcode.com/problems/reverse-bits

pub fn reverse_bits(mut x: u32) -> u32 {
    // This has built-in support already...
    // x.reverse_bits()
    let mut result = 0;
    for _ in 0..32 {
        result <<= 1;
        result |= x & 1;
        x >>= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            reverse_bits(0b00000010100101000001111010011100),
            0b00111001011110000010100101000000
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            reverse_bits(0b11111111111111111111111111111101),
            0b10111111111111111111111111111111
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(reverse_bits(0), 0);
    }
}
