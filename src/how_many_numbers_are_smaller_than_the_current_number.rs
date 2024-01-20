// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut count = vec![0; 101];
    for &i in nums.iter() {
        count[i as usize] += 1;
    }
    for i in 1..=100 {
        count[i] += count[i - 1]
    }
    let mut result = Vec::new();
    for i in nums {
        result.push(if i == 0 { 0 } else { count[i as usize - 1] });
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }
    #[test]
    fn test_4() {
        assert_eq!(
            smaller_numbers_than_current(vec![5, 0, 10, 0, 10, 6]),
            vec![2, 0, 4, 0, 4, 3]
        );
    }
}
