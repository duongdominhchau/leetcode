// https://leetcode.com/problems/find-all-duplicates-in-an-array/

pub fn find_duplicates_sign_flip(mut nums: Vec<i32>) -> Vec<i32> {
    // Use `abs(nums[i]) - 1` as index, on first occurence, flip the number at that index.
    // If we see a negative number at the target index, we know nums[i] appeared twice
    let mut result = Vec::new();
    for i in 0..nums.len() {
        let index = (nums[i].abs() - 1) as usize;
        if nums[index] < 0 {
            result.push(nums[i].abs());
        }
        nums[index as usize] *= -1;
    }
    result
}

pub fn find_duplicates_stacked_count(mut nums: Vec<i32>) -> Vec<i32> {
    let p = 1_000_000;
    for i in 0..nums.len() {
        // Use `nums[index]` to count the number of occurences of index + 1, each occurence
        // increase the value by 10^6. This is safe because each value appear at most twice,
        // and n <= 10^5 so the largest number is 10^5, it will be discarded when we divide
        let val = nums[i] % p;
        nums[(val - 1) as usize] += p;
    }
    nums.into_iter()
        .enumerate()
        // Find numbers appeared twice
        .filter(|(_index, val)| val / p == 2)
        // Map from the range 0..n back to 1..=n
        .map(|(index, _)| index as i32 + 1)
        .collect()
}

pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    find_duplicates_sign_flip(nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![2, 3]);
    }
    #[test]
    fn test_2() {
        assert_eq!(find_duplicates(vec![1, 1, 2]), vec![1]);
    }
    #[test]
    fn test_3() {
        assert_eq!(find_duplicates(vec![1]), Vec::<i32>::new());
    }
}
