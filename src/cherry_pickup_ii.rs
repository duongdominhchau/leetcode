// https://leetcode.com/problems/cherry-pickup-ii/

pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    // The robots can only move downward, so the actual problem is find the
    // optimal position for both robots on each row
    let rows = grid.len();
    let cols = grid[0].len();

    // Edge case: Shared path between both robots
    if cols == 1 {
        return grid.into_iter().map(|row| row[0]).sum();
    }

    // R1 = Robot 1, R2 = Robot 2
    // best[row][p1][p2] = max value obtained when placing R1 at p1 and R2 at p2.
    // We assume p1 < p2 because when both stand on the same cell, only one increase the sum, the
    // other robot wasted a move, and if R2 can move to the left of R1, we can just let R1 takes
    // that position and use R2 to take the position of R1 instead, that will shorten the path
    // required by both robots
    //
    // Need to initialize with -1 because 0 is a valid value to take. If we initialize to 0, we
    // can't distinguish between invalid move and valid move to cell without cherries
    let mut best = vec![vec![vec![-1; cols]; cols]; rows];
    // There is no other choice for the first row
    best[0][0][cols - 1] = grid[0][0] + grid[0][cols - 1];
    // For each other row
    for row in 1..rows {
        // Try all possible combinations of (p1, p2)
        for p1 in 0..cols {
            // We assumed R2 is to the right of R1, so we start the search from p1 + 1
            for p2 in (p1 + 1)..cols {
                let mut current_best = -1;
                // There may be up to 9 (3x3) previous position combinations
                for prev_p1 in p1.saturating_sub(1)..=(p1 + 1).min(cols - 1) {
                    for prev_p2 in p2.saturating_sub(1)..=(p2 + 1).min(cols - 1) {
                        current_best = current_best.max(best[row - 1][prev_p1][prev_p2]);
                    }
                }
                // Need to check this to ensure invalid moves will have -1 as its best value.
                // Without this, an invalid move to a cell with large number of cherries will
                // mess up subsequent max calculations
                if current_best != -1 {
                    // We move both robots at once
                    best[row][p1][p2] = current_best + grid[row][p1] + grid[row][p2];
                }
            }
        }
    }

    let mut result = 0;
    for p1 in 0..cols {
        for p2 in (p1 + 1)..cols {
            result = result.max(best[rows - 1][p1][p2]);
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
            cherry_pickup(vec![
                vec![3, 1, 1],
                vec![2, 5, 1],
                vec![1, 5, 5],
                vec![2, 1, 1]
            ]),
            24
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            cherry_pickup(vec![
                vec![1, 0, 0, 0, 0, 0, 1],
                vec![2, 0, 0, 0, 0, 3, 0],
                vec![2, 0, 9, 0, 0, 0, 0],
                vec![0, 3, 0, 5, 4, 0, 0],
                vec![1, 0, 2, 3, 0, 0, 6]
            ]),
            28
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            cherry_pickup(vec![
                vec![0, 8, 7, 10, 9, 10, 0, 9, 6],
                vec![8, 7, 10, 8, 7, 4, 9, 6, 10],
                vec![8, 1, 1, 5, 1, 5, 5, 1, 2],
                vec![9, 4, 10, 8, 8, 1, 9, 5, 0],
                vec![4, 3, 6, 10, 9, 2, 4, 8, 10],
                vec![7, 3, 2, 8, 3, 3, 5, 9, 8],
                vec![1, 2, 6, 5, 6, 2, 0, 10, 0]
            ]),
            96
        );
    }
    #[test]
    fn test_4() {
        assert_eq!(
            cherry_pickup(vec![
                vec![0, 0, 10, 2, 8, 4, 0],
                vec![7, 9, 3, 5, 4, 8, 3],
                vec![6, 9, 8, 3, 5, 6, 0],
                vec![0, 4, 1, 1, 9, 3, 7],
                vec![5, 6, 9, 8, 8, 10, 10],
                vec![9, 2, 9, 7, 4, 8, 3],
                vec![1, 6, 1, 2, 0, 9, 9]
            ]),
            96
        );
    }
    #[test]
    fn test_5() {
        assert_eq!(cherry_pickup(vec![vec![1], vec![2], vec![3],]), 6);
    }
}
