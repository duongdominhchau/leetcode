// https://leetcode.com/problems/edit-distance/

pub fn min_distance(word1: String, word2: String) -> i32 {
    let (n1, n2) = (word1.len(), word2.len());
    let s1: Vec<char> = word1.chars().collect();
    let s2: Vec<char> = word2.chars().collect();
    // edit[i][j] = number of operations needed to create
    // word2[..j] from word1[..i]
    let mut edit = vec![vec![i32::MAX; n2 + 1]; n1 + 1];

    // Need to apply delete all the way to reach empty string
    for i in 0..=n1 {
        edit[i][0] = i as i32;
    }
    // Similarly, need to insert all the way to build the full string
    for i in 0..=n2 {
        edit[0][i] = i as i32;
    }

    for i in 1..=n1 {
        for j in 1..=n2 {
            edit[i][j] = if s1[i - 1] == s2[j - 1] {
                // If current character matches, we have the option to do nothing
                edit[i - 1][j - 1]
            } else {
                // Apply 1 operation: Replace, Insert, Delete respectively
                1 + edit[i - 1][j - 1].min(edit[i - 1][j]).min(edit[i][j - 1])
            };
        }
    }
    edit[n1][n2]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: &str, b: &str, expected: i32) {
        assert_eq!(min_distance(a.to_string(), b.to_string()), expected);
    }

    #[test]
    fn test_1() {
        check("horse", "ros", 3);
    }
    #[test]
    fn test_2() {
        check("intention", "execution", 5);
    }
    #[test]
    fn test_3() {
        check("a", "ab", 1);
    }
}
