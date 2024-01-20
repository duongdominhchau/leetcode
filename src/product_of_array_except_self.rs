// https://leetcode.com/problems/product-of-array-except-self/
pub fn product_except_self_1(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    // Track accumulated product from both sides
    let mut prefix = vec![1; n];
    let mut suffix = vec![1; n];
    prefix[0] = nums[0];
    suffix[n - 1] = nums[n - 1];
    for i in 1..n {
        prefix[i] = prefix[i - 1] * nums[i];
        let right_index = n - 1 - i;
        suffix[right_index] = suffix[right_index + 1] * nums[right_index];
    }
    (0..n)
        .map(|i| {
            // When at the edge, take the product from the other side
            if i == 0 {
                suffix[1]
            } else if i == n - 1 {
                prefix[n - 2]
            // Otherwise, take from both side
            } else {
                prefix[i - 1] * suffix[i + 1]
            }
        })
        .collect()
}

// Space-optimized implementation
pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![1; n];
    // We put the prefix product in result first
    // result[i] = product of numbers before position i
    result[1] = nums[0];
    for i in 1..n {
        result[i] = result[i - 1] * nums[i - 1];
    }
    // At this point, only result[n-1] is correct. Next we need to fix other elements.
    // We know from the previous implementation that result[i] = prefix[i-1] * suffix[i+1]
    // If we iterate backward, result[i] is prefix[i-1], we just need to track suffix[i+1]
    // As this is the final pass, we can directly modify the input list to store that
    for i in (0..(n - 1)).rev() {
        // result[i] is prefix[i-1] already, we just need to multiply suffix[i+1]
        // nums[i] serves as suffix[i]
        result[i] *= nums[i + 1];
        nums[i] *= nums[i + 1];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }
    #[test]
    fn test_2() {
        assert_eq!(
            product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
