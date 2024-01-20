// https://leetcode.com/problems/add-digits

pub fn add_digits(num: i32) -> i32 {
    let n = num;
    // See: https://en.wikipedia.org/wiki/Digital_root#Congruence_formula
    if n == 0 {
        return 0;
    }
    // Elementary
    if n % 9 == 0 {
        return 9;
    }
    // TODO: Why?
    n % 9
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(add_digits(38), 2);
    }
}
