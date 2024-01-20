// https://leetcode.com/problems/counting-bits/

pub fn count_bits(n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut result = vec![0; n + 1];
    for i in 1..=n {
        result[i] = i.count_ones() as i32;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(count_bits(2), vec![0, 1, 1]);
    }
    #[test]
    fn test_2() {
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
