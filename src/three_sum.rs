// https://leetcode.com/problems/3sum

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();

    nums.sort_unstable();
    let mut result = Vec::new();

    for i in 0..n {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut j = i + 1;
        let mut k = n - 1;
        while j < k {
            // Skip duplicated numbers
            if j - 1 > i && nums[j] == nums[j - 1] {
                j += 1;
                continue;
            }
            if k + 1 < n && nums[k] == nums[k + 1] {
                k -= 1;
                continue;
            }

            if nums[j] + nums[k] > -nums[i] {
                k -= 1;
            } else if nums[j] + nums[k] < -nums[i] {
                j += 1;
            } else {
                result.push(vec![nums[i], nums[j], nums[k]]);
                j += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: Vec<i32>, mut output: Vec<Vec<i32>>) {
        let mut actual = three_sum(input);
        actual.sort_unstable();
        output.sort_unstable();
        assert_eq!(actual, output);
    }

    #[test]
    fn test_1() {
        check(
            vec![-1, 0, 1, 2, -1, -4],
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        );
    }
    #[test]
    fn test_2() {
        check(vec![0, 1, 1], Vec::new());
    }
    #[test]
    fn test_3() {
        check(vec![0, 0, 0], vec![vec![0, 0, 0]]);
    }
    #[test]
    fn test_4() {
        check(
            vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6],
            vec![
                vec![-4, -2, 6],
                vec![-4, 0, 4],
                vec![-4, 1, 3],
                vec![-4, 2, 2],
                vec![-2, -2, 4],
                vec![-2, 0, 2],
            ],
        );
    }
}
