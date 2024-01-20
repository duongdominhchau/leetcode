// https://leetcode.com/problems/find-the-town-judge/

pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    // We define the edge a->b as "a trusts b"
    // `degree` stores (in, out) count. The judge trusts no one and
    // is trusted by everyone else, so it will be (n-1, 0)
    let mut degree = vec![(0, 0); n];
    for t in trust {
        degree[(t[0] - 1) as usize].1 += 1;
        degree[(t[1] - 1) as usize].0 += 1;
    }
    degree
        .into_iter()
        .enumerate()
        .find(|(_, (inbound, outbound))| *inbound == n - 1 && *outbound == 0)
        .map(|(index, _)| (index + 1) as i32)
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_judge(2, vec![vec![1, 2]]), 2);
    }
    #[test]
    fn test_2() {
        assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    }
    #[test]
    fn test_3() {
        assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]), -1);
    }
}
