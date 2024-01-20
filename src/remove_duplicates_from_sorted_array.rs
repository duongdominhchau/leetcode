// https://leetcode.com/problems/remove-duplicates-from-sorted-array

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k = 1;
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            nums[k] = nums[i];
            k += 1;
        }
    }
    k as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2);
    }
    #[test]
    fn test_2() {
        assert_eq!(
            remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
            5
        );
    }
}
