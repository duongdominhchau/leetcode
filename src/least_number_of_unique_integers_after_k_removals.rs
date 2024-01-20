// https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/

pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
    use std::collections::HashMap;
    let mut num_freq = HashMap::new();
    for &x in arr.iter() {
        let count = num_freq.entry(x).or_insert(0);
        *count += 1;
    }
    let mut sorted_frequency: Vec<i32> = num_freq.iter().map(|(_k, v)| *v).collect();
    sorted_frequency.sort_unstable();

    let mut i = 0;
    while i < sorted_frequency.len() && k >= sorted_frequency[i] {
        k -= sorted_frequency[i];
        i += 1;
    }
    sorted_frequency[i..].len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_least_num_of_unique_ints(vec![5, 5, 4], 1), 1);
    }
    #[test]
    fn test_2() {
        assert_eq!(
            find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3),
            2
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(find_least_num_of_unique_ints(vec![2, 1, 1, 3, 3, 3], 3), 1);
    }
}
