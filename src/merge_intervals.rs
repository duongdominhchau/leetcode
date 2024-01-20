// https://leetcode.com/problems/merge-intervals

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_unstable_by_key(|v| v[0]);
    let mut result = vec![intervals[0].clone()];
    for interval in intervals.into_iter().skip(1) {
        let index = result.len() - 1;
        let last_interval = &mut result[index];
        if last_interval[1] >= interval[0] {
            last_interval[1] = interval[1].max(last_interval[1]);
        } else {
            result.push(interval);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);
    }
    #[test]
    fn test_3() {
        assert_eq!(merge(vec![vec![1, 4], vec![2, 3]]), vec![vec![1, 4]]);
    }
}
