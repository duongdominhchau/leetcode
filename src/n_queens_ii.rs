// https://leetcode.com/problems/n-queens/

fn valid_position(board: &Vec<Vec<bool>>, row: usize, col: usize) -> bool {
    // Column check
    for r in 0..=row {
        if board[r][col] {
            return false;
        }
    }
    // Top-left to bottom-right diagonal
    for i in 0..=row.min(col) {
        if board[row - i][col - i] {
            return false;
        }
    }
    // Top-right to bottom-left diagonal
    for i in 0..=row.min(board.len() - 1 - col) {
        if board[row - i][col + i] {
            return false;
        }
    }
    true
}

fn count_solutions(board: &mut Vec<Vec<bool>>, count: &mut i32, row: usize) {
    let n = board.len();
    if row == n {
        *count += 1;
        return;
    }
    for col in 0..n {
        if valid_position(&board, row, col) {
            board[row][col] = true;
            count_solutions(board, count, row + 1);
            board[row][col] = false;
        }
    }
}
pub fn total_n_queens_precomputed(n: i32) -> i32 {
    const RESULT: [i32; 10] = [0, 1, 0, 0, 2, 10, 4, 40, 92, 352];
    RESULT[n as usize]
}
pub fn total_n_queens(n: i32) -> i32 {
    let n = n as usize;
    let mut result = 0;
    count_solutions(&mut vec![vec![false; n]; n], &mut result, 0);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(total_n_queens(1), 1);
    }
    #[test]
    fn test_2() {
        assert_eq!(total_n_queens(4), 2);
    }
}
