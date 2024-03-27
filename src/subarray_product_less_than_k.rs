// https://leetcode.com/problems/subarray-product-less-than-k

pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut count = 0;
    for i in 0..n {
        let mut prod = 1;
        for j in i..n {
            prod *= nums[j];
            if prod >= k {
                break;
            }
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100), 8);
    }
    #[test]
    fn test_2() {
        assert_eq!(num_subarray_product_less_than_k(vec![1, 2, 3], 0), 0);
    }
    #[test]
    fn test_3() {
        assert_eq!(
            num_subarray_product_less_than_k(
                vec![10, 9, 10, 4, 3, 8, 3, 3, 6, 2, 10, 10, 9, 3],
                19
            ),
            18
        );
    }
}
