// https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions

pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;

    let mut occurrences: HashMap<i32, usize> = HashMap::new();
    nums.into_iter().for_each(|n| {
        let count = occurrences.entry(n).or_insert(0);
        *count += 1;
    });

    let mut result = Vec::new();
    let mut all_zero = false;
    while !all_zero {
        all_zero = true;
        let mut row: Vec<i32> = Vec::new();
        for (num, count) in occurrences.iter_mut() {
            if *count > 0 {
                row.push(*num);
                *count -= 1;
                all_zero = false;
            }
        }
        if !all_zero {
            result.push(row);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut actual = find_matrix(vec![1, 3, 4, 1, 2, 3, 1]);
        for subarray in actual.iter_mut() {
            subarray.sort_unstable();
        }
        assert_eq!(actual, vec![vec![1, 2, 3, 4], vec![1, 3], vec![1]]);
    }
}
