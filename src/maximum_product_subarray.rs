// https://leetcode.com/problems/maximum-product-subarray/

pub fn max_product(nums: Vec<i32>) -> i32 {
    // Similar to the Maximum Subarray, but this time we need to track both the largest positive
    // and the smallest negative products
    let mut best_neg = 0;
    let mut best_pos = 0;
    let mut best_prod = 0;
    for &x in nums.iter() {
        if x == 0 {
            // Start new subarray as multiplying by 0 will ruin all products
            best_neg = 0;
            best_pos = 0;
        } else if x > 0 {
            // Positive number, just put into the existing subarrays
            best_pos = if best_pos == 0 { x } else { best_pos * x };
            best_neg = if best_neg == 0 { 0 } else { best_neg * x };
        } else {
            // Negative number, when we take it, the product will have its sign reversed
            let new_best_pos = if best_neg == 0 { 0 } else { best_neg * x };
            let new_best_neg = if best_pos == 0 { x } else { best_pos * x };
            best_pos = new_best_pos;
            best_neg = new_best_neg;
        }
        best_prod = best_prod.max(best_pos);
    }
    if best_prod == 0 {
        nums.into_iter().max().unwrap()
    } else {
        best_prod
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
    }
    #[test]
    fn test_2() {
        assert_eq!(max_product(vec![-2, 0, -1]), 0);
    }
}
