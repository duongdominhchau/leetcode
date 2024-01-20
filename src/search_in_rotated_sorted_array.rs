// https://leetcode.com/problems/search-in-rotated-sorted-array/

fn find_split_point(nums: &[i32]) -> usize {
    // The minimum is after the maximum, so we need to find i where
    // nums[i] < nums[i - 1] and nums[i] < nums[i+1]
    // Note that the array is already sorted, so we only need to check for
    // nums[i] < nums[i - 1]
    let mut left = 0;
    let mut right = nums.len() - 1;
    // No rotation or single-element list
    if nums[left] <= nums[right] {
        return 0;
    }
    while left <= right {
        // Handle 2-element sublist
        if left == right - 1 {
            return if nums[left] < nums[right] {
                left
            } else {
                right
            };
        }
        let mid = left + (right - left) / 2;
        if nums[mid] < nums[mid - 1] {
            return mid;
        }
        if nums[mid] > nums[right] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    unreachable!()
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let k = find_split_point(&nums);
    nums[0..k]
        .binary_search(&target)
        .map(|x| x as i32)
        .unwrap_or_else(|_| {
            nums[k..nums.len()]
                .binary_search(&target)
                .map(|x| (k + x) as i32)
                .unwrap_or(-1)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }
    #[test]
    fn test_2() {
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }
    #[test]
    fn test_3() {
        assert_eq!(search(vec![1], 0), -1);
    }
    #[test]
    fn test_4() {
        assert_eq!(search(vec![3, 1], 0), -1);
    }
    #[test]
    fn test_5() {
        assert_eq!(search(vec![2, 3, 4, 5, 1], 1), 4);
    }
}
