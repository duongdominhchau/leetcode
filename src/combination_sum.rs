// https://leetcode.com/problems/combination-sum

fn generate(result: &mut Vec<Vec<i32>>, current: &mut Vec<i32>, candidates: &[i32], target: i32) {
    if target == 0 {
        result.push(current.clone());
    }
    if target <= 0 {
        return;
    }
    for i in 0..candidates.len() {
        current.push(candidates[i]);
        // Shorten the slice to ensure previous numbers are not picked up anymore. If we have the
        // numbers 1, 2 and the target 3, we don't want it to pick 1 again after reaching 2 because
        // that will yield [1, 2] and [2, 1]. We want combinations, not permutations
        generate(result, current, &candidates[i..], target - candidates[i]);
        current.pop();
    }
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    generate(&mut result, &mut Vec::new(), &candidates, target);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
    }
}
