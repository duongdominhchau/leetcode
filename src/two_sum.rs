use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_indices = HashMap::<i32, Vec<i32>>::new();
    nums.iter()
        .enumerate()
        .for_each(|(i, x)| match num_indices.get_mut(x) {
            Some(indices) => {
                indices.push(i as i32);
            }
            None => {
                num_indices.insert(*x, vec![i as i32]);
            }
        });
    for (x, indices) in num_indices.iter() {
        let complement = target - x;
        if complement == *x {
            if indices.len() < 2 {
                continue;
            }
            return vec![indices[0], indices[1]];
        } else if let Some(other_indices) = num_indices.get(&complement) {
            let mut v = vec![indices[0], other_indices[0]];
            v.sort_unstable();
            return v;
        }
    }
    unreachable!("Wrong problem statement")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
