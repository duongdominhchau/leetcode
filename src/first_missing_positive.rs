// https://leetcode.com/problems/first-missing-positive/

pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    // We need to find the smallest missing positive number. This number must be in 1..=n
    //
    // - If `nums` contains all numbers from 1 upward, the smallest one not in `nums` is `n`
    // - If any number below `n` is missing, then one of these numbers will be answer. Whatever
    // number it is, it will be less than `n`, so again it is in 1..=n
    //
    // To optimize space, we will reuse nums as a hash table, and reorder elements to track if
    // a number in 1..=n exists in the input

    let mut i = 0;
    while i < n {
        let index = i as usize;
        // In short, we want to ensure nums[i] is either i+1 or a number outside 1..=n
        let k = (nums[index] - 1) as usize;
        // Current position and target position may contain the same number.
        if (1..=n).contains(&nums[index]) && nums[index] != nums[k] {
            nums.swap(k, index);
            // The swapped number may be in 1..=n but is still in wrong position, so we only move
            // to the next position once current position is i+1 or value outside 1..=n
            continue;
        }
        i += 1;
    }
    for i in 0..n {
        if nums[i as usize] != i + 1 {
            return i + 1;
        }
    }
    n + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
    }
    #[test]
    fn test_3() {
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
    #[test]
    fn test_4() {
        assert_eq!(first_missing_positive(vec![1, 2, 3]), 4);
    }
    #[test]
    fn test_5() {
        assert_eq!(first_missing_positive(vec![2, 3, 4]), 1);
    }
    #[test]
    fn test_6() {
        assert_eq!(first_missing_positive(vec![1, 1]), 2);
    }
}
