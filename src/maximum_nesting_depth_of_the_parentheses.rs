// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/

pub fn max_depth(s: String) -> i32 {
    let mut depth = 0;
    let mut max_depth = 0;
    for ch in s.chars() {
        match ch {
            '(' => {
                depth += 1;
                max_depth = max_depth.max(depth);
            }
            ')' => {
                depth -= 1;
            }
            _ => {}
        }
    }
    max_depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(max_depth("(1)+((2))+(((3)))".to_string()), 3);
    }
}
