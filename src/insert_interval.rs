// https://leetcode.com/problems/insert-interval/

pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut start = match intervals.binary_search_by_key(&new_interval[0], |v| v[0]) {
        Ok(index) => index,
        Err(index) => index,
    };
    let mut end = start;
    while end < intervals.len() && intervals[end][0] <= new_interval[1] {
        new_interval[1] = new_interval[1].max(intervals[end][1]);
        end += 1;
    }
    // New interval may overlap with the interval before it
    if start > 0 && intervals[start - 1][1] >= new_interval[0] {
        new_interval[0] = intervals[start - 1][0];
        // Maybe `new_interval` is completely in `intervals[start]`, so we need to
        // adjust the end of the merged interval accordingly for later update
        new_interval[1] = new_interval[1].max(intervals[start - 1][1]);
        start -= 1;
    }
    // Overlapped somewhere
    if start != end {
        intervals.drain(start + 1..end);
        intervals[start] = new_interval;
    } else {
        intervals.insert(start, new_interval);
    }
    intervals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_interval_rhs_overlap_at_the_beginning_of_existing_interval() {
        assert_eq!(
            insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
    #[test]
    fn test_new_interval_overlaps_at_both_ends() {
        assert_eq!(
            insert(vec![vec![3, 4], vec![5, 6]], vec![2, 5]),
            vec![vec![2, 6]]
        );
    }
    #[test]
    fn test_new_interval_rhs_extend_over_the_rightmost_existing_interval() {
        assert_eq!(
            insert(vec![vec![1, 2], vec![3, 4]], vec![2, 5]),
            vec![vec![1, 5]]
        );
    }
    #[test]
    fn test_new_interval_immediately_follow_existing_interval() {
        assert_eq!(insert(vec![vec![1, 2]], vec![2, 5]), vec![vec![1, 5]]);
    }
    #[test]
    fn test_new_interval_equals_multiple_existing_intervals_merged() {
        assert_eq!(insert(vec![vec![1, 2]], vec![2, 5]), vec![vec![1, 5]]);
    }
    #[test]
    fn test_new_interval_fits_into_existing_interval() {
        assert_eq!(insert(vec![vec![1, 4]], vec![2, 3]), vec![vec![1, 4]]);
    }
    #[test]
    fn test_new_interval_is_at_the_beginning_and_no_overlap() {
        assert_eq!(
            insert(vec![vec![2, 3]], vec![0, 1]),
            vec![vec![0, 1], vec![2, 3]]
        );
    }
    #[test]
    fn test_new_interval_is_at_the_end_and_no_overlap() {
        assert_eq!(
            insert(vec![vec![2, 3]], vec![4, 5]),
            vec![vec![2, 3], vec![4, 5]]
        );
    }
    #[test]
    fn test_empty_interval_list() {
        assert_eq!(insert(Vec::new(), vec![2, 3]), vec![vec![2, 3]]);
    }
    #[test]
    fn test_single_element_new_interval() {
        assert_eq!(
            insert(vec![vec![3, 5], vec![12, 15]], vec![6, 6]),
            vec![vec![3, 5], vec![6, 6], vec![12, 15]]
        );
    }
}
