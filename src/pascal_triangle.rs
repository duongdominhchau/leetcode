// https://leetcode.com/problems/pascals-triangle

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut result = vec![vec![1]];
    for k in 2..=(num_rows as usize) {
        let prev = &result[result.len() - 1];
        let mut v = vec![1];
        for i in 1..(k - 1) {
            v.push(prev[i - 1] + prev[i]);
        }
        v.push(1);
        result.push(v);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(generate(1), vec![vec![1]]);
    }
}
