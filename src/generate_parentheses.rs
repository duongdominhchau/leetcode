// https://leetcode.com/problems/generate-parentheses

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result = Vec::new();
    if n == 0 {
        return result;
    }
    // Try placing opening and closing parentheses according to the rule,
    // then collect all valid combinations
    generate_parenthesis_inner(&mut String::new(), &mut result, n, n);
    result
}

fn generate_parenthesis_inner(s: &mut String, result: &mut Vec<String>, open: i32, closed: i32) {
    if open == 0 && closed == 0 {
        result.push(s.clone());
    }
    if open > 0 {
        s.push('(');
        generate_parenthesis_inner(s, result, open - 1, closed);
        s.pop();
    }
    // Only add closing parenthesis if there is unclosed opening parenthesis
    if open < closed {
        s.push(')');
        generate_parenthesis_inner(s, result, open, closed - 1);
        s.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(generate_parenthesis(0), Vec::<String>::new());
    }
    #[test]
    fn test_2() {
        assert_eq!(generate_parenthesis(1), vec!["()"]);
    }
    #[test]
    fn test_3() {
        assert_eq!(generate_parenthesis(2), vec!["(())", "()()"]);
    }
    #[test]
    fn test_4() {
        assert_eq!(
            generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
    #[test]
    fn test_5() {
        assert_eq!(
            generate_parenthesis(4),
            vec![
                "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
                "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"
            ]
        );
    }
}
