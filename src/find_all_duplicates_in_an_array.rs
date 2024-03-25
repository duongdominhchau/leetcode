// https://leetcode.com/problems/find-all-duplicates-in-an-array/

pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
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
