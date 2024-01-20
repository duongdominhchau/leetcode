// https://leetcode.com/problems/move-zeroes

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut k = 0;
    for i in 0..nums.len() {
        nums[k] = nums[i];
        if nums[k] != 0 {
            k += 1;
        }
    }
    for i in k..nums.len() {
        nums[i] = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut a = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut a);
        assert_eq!(a, vec![1, 3, 12, 0, 0]);
    }
    #[test]
    fn test_2() {
        let mut a = vec![0];
        move_zeroes(&mut a);
        assert_eq!(a, vec![0]);
    }
}
