// https://leetcode.com/problems/combination-sum-iv/

pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums: Vec<usize> = nums
        .into_iter()
        .filter(|&k| k <= target)
        .map(|k| k as usize)
        .collect();
    nums.sort_unstable_by(|a, b| a.cmp(b).reverse());
    let target = target as usize;

    // count[i] = number of combinations that yields i as the sum
    // count[i] = 0 means it is impossible to create a combination of sum i
    let mut count = vec![0; target + 1];
    // Need this so `count[k]` is correctly counted as `1` for every `k` in `nums`
    count[0] = 1;
    for i in 1..=target {
        for k in nums.iter().filter(|&&k| k <= i) {
            count[i] += count[i - k];
        }
    }
    count[target]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(combination_sum4(vec![1, 2, 3], 4), 7);
    }
    #[test]
    fn test_2() {
        assert_eq!(combination_sum4(vec![9], 3), 0);
    }
}
