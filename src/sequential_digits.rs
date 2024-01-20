// https://leetcode.com/problems/sequential-digits

fn ten_pow(k: usize) -> i32 {
    (0..k).fold(1, |acc, _| acc * 10)
}

/// Return the most significant digit and its position
/// The right most digit is digit 0
fn most_significant_digit(mut value: i32) -> (i32, i32) {
    let mut position = 0;
    while value > 10 {
        value /= 10;
        position += 1;
    }
    (value, position)
}

fn gen_num(start_digit: i32, len: i32) -> Option<i32> {
    if start_digit + len - 1 > 9 {
        return None;
    }
    let n = (0..len)
        .map(|i| start_digit + i)
        .fold(0, |acc, current| acc * 10 + current);
    Some(n)
}

/// Find n >= value where n satisfy problem condition
fn next_num(value: i32) -> Option<i32> {
    if value > 123_456_789 {
        return None;
    }
    let (msd, position) = most_significant_digit(value);
    // position counts from 0, so the number length is position + 1
    if let Some(n) = gen_num(msd, position + 1) {
        if n >= value {
            return Some(n);
        }
    }
    next_num((msd + 1) * ten_pow(position as usize))
}

pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = low;
    while let Some(value) = next_num(current) {
        if value > high {
            break;
        }
        result.push(value);
        current = value + 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(sequential_digits(100, 300), vec![123, 234]);
    }
    #[test]
    fn test_2() {
        assert_eq!(
            sequential_digits(1000, 13000),
            vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            sequential_digits(10, 1000000000),
            vec![
                12, 23, 34, 45, 56, 67, 78, 89, 123, 234, 345, 456, 567, 678, 789, 1234, 2345,
                3456, 4567, 5678, 6789, 12345, 23456, 34567, 45678, 56789, 123456, 234567, 345678,
                456789, 1234567, 2345678, 3456789, 12345678, 23456789, 123456789
            ]
        );
    }
    #[test]
    fn test_4() {
        assert_eq!(sequential_digits(58, 155), vec![67, 78, 89, 123]);
    }
}
