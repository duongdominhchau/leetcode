// https://leetcode.com/problems/fizz-buzz/
pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n)
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            (_, _) => i.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            fizz_buzz(3),
            vec!["1".to_string(), "2".to_string(), "Fizz".to_string()]
        );
    }
}
