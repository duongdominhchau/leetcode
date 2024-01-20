// https://leetcode.com/problems/pascals-triangle-ii

pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut n = row_index;
    let mut prev = vec![1];
    while n != 0 {
        // Build new row
        let mut current = vec![1];
        for i in 1..prev.len() {
            current.push(prev[i - 1] + prev[i]);
        }
        current.push(1);
        // Switch
        prev = current;
        n -= 1;
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    }
    #[test]
    fn test_2() {
        assert_eq!(get_row(0), vec![1]);
    }
    #[test]
    fn test_3() {
        assert_eq!(get_row(1), vec![1, 1]);
    }
}
