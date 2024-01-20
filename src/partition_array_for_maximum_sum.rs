// https://leetcode.com/problems/partition-array-for-maximum-sum

pub fn max_sum_after_partitioning_original_solution(arr: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    // `best[i]` is the largest sum we can get if the last slice ends at index `i`
    let mut best_sum = vec![0; arr.len()];

    // Calculate optimal solution for the single-slice situation first
    let mut max_value = arr[0];
    for i in 0..k.min(arr.len()) {
        let slice_len = (i + 1) as i32;
        max_value = max_value.max(arr[i]);
        best_sum[i] = max_value * slice_len;
    }

    // For each position
    for i in 1..arr.len() {
        // Try all possible slice length
        for slice_len in 1..=(k as usize) {
            // Skip this length as taking a slice of this length means there will be no element for
            // another slice. That contradicts with our assumption that this is NOT the first slice
            // (we rely on the optimal result of the previous subproblem, so we don't consider the
            // first slice here)
            let start = i + 1 - slice_len;
            let end = i;
            if start <= 0 {
                break;
            }
            let slice = &arr[start..=end];
            // Slice of length `s` requires `s` elements.
            // All elements in the slice will be replaced by the max value, so this slice will have
            // a sum of `s * slice.max()`
            // We took `s` elements out, so the previous subproblem is at `i - s`
            // => best[i] = best[i - s] + s * slice.max() for all valid s
            // Just pick the s yielding largest best[i]
            let slice_sum = slice.iter().max().unwrap() * slice_len as i32;
            let current = best_sum[i - slice_len] + slice_sum;
            if current > best_sum[i] {
                best_sum[i] = current;
            }
        }
    }
    best_sum[arr.len() - 1]
}

pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    // `best[i]` is the largest sum we can get if the last slice ends at index `i-1`
    // We need this awkward indexing scheme to avoid handling the first slice
    // Trivially, best[0] is 0 because there is no slice before the first element.
    let mut best_sum = vec![0; arr.len() + 1];

    // For each position
    for i in 0..arr.len() {
        // Try creating a slice with largest value after transformation
        let mut max_value = 0;
        let mut max_sum = 0;

        // Here we try all possible slice length.
        // Because slice of length 2 will include slice of length 1, we will start with the
        // shortest slice first to avoid recalculation
        //
        // j is not the slice length, it is the offset to the first element in the slice.
        // i-j is where the slice starts and i is where the slice ends.
        for j in 0..k.min(i + 1) {
            // max value from i - j + 1 to i has already been calculated in previous iteration
            max_value = max_value.max(arr[i - j]);
            let slice_len = (j + 1) as i32;
            let current_slice_sum = max_value * slice_len;
            // Current slice starts at i - j, so previous subproblem is at i - j - 1.
            // However, we use 1-based index for this solution array while everything else
            // uses 0-based index, so the index being used here is just i - j
            let previous_best_sum = best_sum[i - j];
            max_sum = max_sum.max(current_slice_sum + previous_best_sum);
        }
        best_sum[i + 1] = max_sum;
    }
    best_sum[arr.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3),
            84
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            max_sum_after_partitioning(vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4),
            83
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(max_sum_after_partitioning(vec![1], 1), 1);
    }
    #[test]
    fn test_4() {
        assert_eq!(max_sum_after_partitioning(vec![3, 7], 2), 14);
    }
}
