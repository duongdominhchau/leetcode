// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix

pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;
    for row in 0..rows {
        // Sorted already, so all the remaining numbers are negative as well
        if grid[row][0] < 0 {
            count += (rows - row) * cols;
            break;
        }
        for col in 0..cols {
            if grid[row][col] < 0 {
                count += cols - col;
                break;
            }
        }
    }
    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3]
            ]),
            8
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(count_negatives(vec![vec![3, 2], vec![1, 0],]), 0);
    }
}
