// https://leetcode.com/problems/majority-element

pub fn majority_element_naive(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    nums.sort_unstable();

    let mut count = 0;
    let mut num = nums[0];
    for x in nums.into_iter() {
        // There can only exist 1 majority element as it takes more than 50% the space already.
        // Should there be another majority element, it would add up to over 100%.
        if x == num {
            count += 1;
        } else {
            count = 1;
            num = x;
        }
        if count > n / 2 {
            return num;
        }
    }
    num
}
pub fn majority_element_boyer_moore_majority_vote(nums: Vec<i32>) -> i32 {
    // This solution use Boyer--Moore majority vote algorithm. This algorithm relies on the fact
    // that the majority element appears more than n/2 times, so every time the counter reaches 0,
    // there will still be more instances of the majority element in the remaining of the list.
    // With that, eventually the majority element will be tracked and the counter will not reach 0
    // anymore.
    let mut num = nums[0];
    let mut count = 0;
    for x in nums.into_iter() {
        if num == x {
            count += 1;
        } else {
            count -= 1;
        }
        // Switch to track another number
        if count == 0 {
            count = 1;
            num = x;
        }
    }
    num
}

pub fn majority_element_middle_sorted_list(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    // There are >= n/2 instances of the majority element. After sorting, the substring containing
    // these instances will have the length of at least n/2 + 1.
    //
    // - If that substring is at the beginning, we will have a[k] = majority element for k from 0
    // to n/2. Clearly a[n/2] is the majority element.
    // - If we put that substring at the end, we will have a[k] = majority element for k from
    // n-1 down to n-1 - n/2 => a[k] = majority element for k from n/2 - 1 to n - 1. In this case,
    // a[n/2] is also the majority element.
    // - Any other place to put the substring of majority element will have these occurences more
    // centered around n/2, so we don't need to consider them.
    nums[nums.len() / 2]
}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    majority_element_middle_sorted_list(nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
    #[test]
    fn test_3() {
        assert_eq!(majority_element(vec![2]), 2);
    }
    #[test]
    fn test_4() {
        assert_eq!(majority_element(vec![1, 1]), 1);
    }
}
