// https://leetcode.com/problems/unique-paths/description/

pub fn unique_paths_dp(m: i32, n: i32) -> i32 {
    let (rows, cols) = (m as usize, n as usize);
    let mut count = vec![vec![0; cols]; rows];
    for j in 0..cols {
        count[0][j] = 1;
    }
    for i in 0..rows {
        count[i][0] = 1;
    }

    for i in 1..rows {
        for j in 1..cols {
            count[i][j] = count[i - 1][j] + count[i][j - 1];
        }
    }
    count[rows - 1][cols - 1]
}
pub fn unique_paths_math(m: i32, n: i32) -> i32 {
    let (rows, cols) = (m as usize, n as usize);
    // The path is shortest, so we will have `rows - 1` step downward and
    // `cols - 1` steps rightward
    //
    // We need to determine how to arrange these steps. Once all the downward steps are chosen,
    // we have no other choice than fill the rest with rightward steps. To count the number of
    // ways to places these `rows - 1` steps downward. Therefore, all we need to calculate is:
    // C(rows - 1 + col - 1, rows - 1)
    let n = (rows - 1 + cols - 1) as i32;
    let k = (rows - 1).min(cols - 1) as i32;

    // We don't use the formular C(n, k) = n! / (k! * (n-k)!), we will count directly
    // using the formular C(n,k) = (n * (n-1) * ... * (n - k + 1)) / k!
    let p = ((n - k + 1)..=n).fold(1_i128, |acc, cur| acc * cur as i128);
    let count = p / (1..=k).fold(1_i128, |acc, cur| acc * cur as i128);
    count as i32
}

pub fn unique_paths(m: i32, n: i32) -> i32 {
    unique_paths_math(m, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(unique_paths(3, 7), 28);
    }
    #[test]
    fn test_2() {
        assert_eq!(unique_paths(3, 2), 3);
    }
    #[test]
    fn test_3() {
        assert_eq!(unique_paths(10, 10), 48620);
    }
    #[test]
    fn test_4() {
        assert_eq!(unique_paths(23, 12), 193536720);
    }
    #[test]
    fn test_5() {
        assert_eq!(unique_paths(16, 16), 155117520);
    }
}
