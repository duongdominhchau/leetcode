// https://leetcode.com/problems/permutations

fn generate(result: &mut Vec<Vec<i32>>, current: &mut Vec<i32>, nums: &[i32]) {
    let n = nums.len();
    if current.len() == n {
        result.push(current.clone());
        return;
    }
    for i in 0..n {
        if !current.contains(&nums[i]) {
            current.push(nums[i]);
            generate(result, current, nums);
            current.pop();
        }
    }
}
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    generate(&mut result, &mut Vec::new(), &nums);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
