// https://leetcode.com/problems/summary-ranges

pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let n = nums.len();
    let mut result = Vec::new();
    let mut start = 0;
    let mut end = start;
    while end < n {
        while end + 1 < n && nums[end + 1] == nums[end] + 1 {
            end += 1;
        }
        if start == end {
            result.push(nums[start].to_string());
        } else {
            result.push(format!("{}->{}", nums[start], nums[end]));
        }
        start = end + 1;
        end = start;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: Vec<i32>, expected: Vec<&str>) {
        let expected: Vec<String> = expected.into_iter().map(|x| x.to_string()).collect();
        assert_eq!(summary_ranges(input), expected);
    }

    #[test]
    fn test_1() {
        check(vec![0, 1, 2, 4, 5, 7], vec!["0->2", "4->5", "7"]);
    }
    #[test]
    fn test_2() {
        check(vec![0, 2, 3, 4, 6, 8, 9], vec!["0", "2->4", "6", "8->9"]);
    }
}
