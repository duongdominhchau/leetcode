// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits

pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
    arr.sort_unstable_by(|a, b| a.count_ones().cmp(&b.count_ones()).then_with(|| a.cmp(b)));
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }
}
