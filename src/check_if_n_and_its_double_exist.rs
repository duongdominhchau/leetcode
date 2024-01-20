// https://leetcode.com/problems/check-if-n-and-its-double-exist

pub fn check_if_exist(arr: Vec<i32>) -> bool {
    use std::collections::HashMap;
    let mut nums = HashMap::new();
    for &x in arr.iter() {
        let count = nums.entry(x).or_insert(0);
        *count += 1;
    }
    arr.into_iter()
        .find(|&x| {
            if x == 0 {
                nums[&0] >= 2
            } else {
                nums.contains_key(&(2 * x))
            }
        })
        .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(check_if_exist(vec![10, 2, 5, 3]), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(check_if_exist(vec![3, 1, 7, 11]), false);
    }
    #[test]
    fn test_3() {
        assert_eq!(check_if_exist(vec![-2, 0, 10, -19, 4, 6, -8]), false);
    }
}
