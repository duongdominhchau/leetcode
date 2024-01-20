// https://leetcode.com/problems/largest-divisible-subset/

pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    nums.sort_unstable();

    // Track the largest group size we can reach
    let mut group_size = vec![0; n];
    // This list serves two purposes: It tracks the largest element in the group
    // (because we sorted the input) and also enabling tracing back to previous
    // element in the group.
    // We only track the largest element because for every pair (a, b) in the group
    // where a >= b, we have a % b == 0. If a number is a multiple of the largest
    // number, it must be divisible by the smaller numbers in this group.
    let mut prev_index = vec![usize::MAX; n];

    for i in 0..n {
        // Start with the worst choice: group of current element only
        group_size[i] = 1;

        // Then check existing groups to see if current element can fit in
        for j in 0..i {
            let prev_num = nums[j];
            // If it can be added to the group and this group is larger than the current best
            if nums[i] % prev_num == 0 && group_size[j] >= group_size[i] {
                group_size[i] = group_size[j] + 1;
                prev_index[i] = j;
            }
        }
    }

    let max_len = group_size.iter().max().unwrap();
    let mut k = group_size.iter().position(|len| len == max_len).unwrap();
    let mut result = Vec::new();
    while k != usize::MAX {
        result.push(nums[k]);
        k = prev_index[k];
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(largest_divisible_subset(vec![1, 2, 3]), vec![1, 2]);
    }
    #[test]
    fn test_2() {
        assert_eq!(largest_divisible_subset(vec![1, 2, 4, 8]), vec![1, 2, 4, 8]);
    }
    #[test]
    fn test_3() {
        assert_eq!(largest_divisible_subset(vec![1]), vec![1]);
    }
    #[test]
    fn test_4() {
        assert_eq!(largest_divisible_subset(vec![2, 3]), vec![2]);
    }
    #[test]
    fn test_5() {
        assert_eq!(largest_divisible_subset(vec![2, 3, 4, 9, 8]), vec![2, 4, 8]);
    }
}
