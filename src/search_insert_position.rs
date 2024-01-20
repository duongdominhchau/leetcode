// https://leetcode.com/problems/search-insert-position

pub fn search_insert_std(nums: Vec<i32>, target: i32) -> i32 {
    let index = match nums.binary_search(&target) {
        Ok(index) => index,
        Err(index) => index,
    };
    index as i32
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut left = 0;
    let mut right = (n - 1) as i32;
    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    }
    #[test]
    fn test_2() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    }
    #[test]
    fn test_3() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }
    #[test]
    fn test_4() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
