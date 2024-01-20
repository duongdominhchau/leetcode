// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array

pub fn find_min(nums: Vec<i32>) -> i32 {
    // The minimum is after the maximum, so we need to find i where
    // nums[i] < nums[i - 1] and nums[i] < nums[i+1]
    // Note that the array is already sorted, so we only need to check for
    // nums[i] < nums[i - 1]
    let mut left = 0;
    let mut right = nums.len() - 1;
    // No rotation or single-element list
    if nums[left] <= nums[right] {
        return nums[0];
    }
    while left <= right {
        // Handle 2-element sublist
        if left == right - 1 {
            return if nums[left] < nums[right] {
                nums[left]
            } else {
                nums[right]
            };
        }
        let mid = left + (right - left) / 2;
        if nums[mid] < nums[mid - 1] {
            return nums[mid];
        }
        if nums[mid] > nums[right] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
    }
    #[test]
    fn test_2() {
        assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }
    #[test]
    fn test_3() {
        assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
    }
}
