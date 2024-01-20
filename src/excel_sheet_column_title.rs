// https://leetcode.com/problems/excel-sheet-column-title

pub fn convert_to_title(column_number: i32) -> String {
    // This is basically base conversion, but instead of a base with digits, we have A, B, C, etc.
    // as digits
    let base = 'Z' as i32 - 'A' as i32 + 1;
    let mut n = column_number;
    let mut result = String::new();
    while n != 0 {
        let code = (n - 1) % base;
        result.push(('A' as u8 + code as u8) as char);
        n = (n - 1) / base;
    }
    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(convert_to_title(1), "A".to_string());
    }
    #[test]
    fn test_2() {
        assert_eq!(convert_to_title(28), "AB".to_string());
    }
    #[test]
    fn test_3() {
        assert_eq!(convert_to_title(701), "ZY".to_string());
    }
}
