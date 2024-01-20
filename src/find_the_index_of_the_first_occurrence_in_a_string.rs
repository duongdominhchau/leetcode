pub fn str_str(haystack: String, needle: String) -> i32 {
    haystack.find(&needle).map(|x| x as i32).unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(str_str("haystack".to_string(), "needle".to_string()), -1);
    }

    #[test]
    fn test_2() {
        assert_eq!(str_str("haystack".to_string(), "hay".to_string()), 0);
    }
}
