// https://leetcode.com/problems/bitwise-and-of-numbers-range

pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    // 2^k & (2^k-1) is 0 (e.g: 2^3 is 0b1000 with 3 0s while 2^3 - 1 is 0b111 with 3 1s).
    // Therefore, everytime we get past a power of 2, all the bits to the right become 0.
    // Bit k is also flipped from 0 to 1, so it will also be 0 in the final result.
    //
    // If we have 2^k consecutive numbers, their AND will have k zeroes at the end
    // because you can only have 2^k different bit strings of length k, and here we
    // have 2^k consecutive number, so one of them must have all 0s at the end.
    // Therefore, we can find the largest 2^k within range and use it to determine
    // how many 0s at the end.
    //
    // More observation: If there is 2^k between `left` and `right`, then `result`
    // must be 0. Therefore, we don't really need to pre-calculate `left & right`.
    // If there is 2^k, the loop will clear result to 0, and if there is none,
    // these bits will be common between `left` and `right`
    let mut result = left;
    let mut i = 1;
    for _ in 1..31 {
        i <<= 1;
        if left > right - i {
            break;
        }
        // i is 2^k, i-1 has k 1 bits to the right, !(i-1) has k 0 bits to the right
        result &= !(i - 1) & !i;
    }
    result
}
pub fn range_bitwise_and_kernighan(mut left: i32, mut right: i32) -> i32 {
    // Based on observation above, we can conclude the result is the common prefix.
    // We can remove all the different bits until two numbers are the same, that's
    // the common prefix. That's the Kerninghan algorithm.
    let mut count = 0;
    while left < right {
        left >>= 1;
        right >>= 1;
        count += 1;
    }
    left << count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // 101
        // 111
        assert_eq!(range_bitwise_and(5, 7), 4);
    }
    #[test]
    fn test_2() {
        assert_eq!(range_bitwise_and(0, 0), 0);
    }
    #[test]
    fn test_3() {
        // 1111111111111111111111111111111
        // 0000000000000000000000000000001
        assert_eq!(range_bitwise_and(1, 2147483647), 0);
    }
    #[test]
    fn test_4() {
        // 011
        // 110
        assert_eq!(range_bitwise_and(3, 6), 0);
    }
    #[test]
    fn test_5() {
        // 1111111111111111111111111111111
        // 1111111111111111111111111111110
        assert_eq!(range_bitwise_and(2147483646, 2147483647), 2147483646);
    }
}
