// https://leetcode.com/problems/n-queens/

fn can_place(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let n = board.len();
    // Check vertical
    for i in 0..row {
        if board[i][col] == 'Q' {
            return false;
        }
    }
    // Check top-left diagonal
    let mut r = row;
    let mut c = col;
    while r > 0 && c > 0 {
        r -= 1;
        c -= 1;
        if board[r][c] == 'Q' {
            return false;
        }
    }
    // Check top-right diagonal
    let mut r = row;
    let mut c = col;
    while r > 0 && c < n - 1 {
        r -= 1;
        c += 1;
        if board[r][c] == 'Q' {
            return false;
        }
    }
    true
}

fn place(row: usize, board: &mut Vec<Vec<char>>, result: &mut Vec<Vec<String>>) {
    let n = board.len();
    if row == n {
        let mut solution = Vec::new();
        for r in board.iter() {
            let mut row_str = String::new();
            r.iter().for_each(|&ch| row_str.push(ch));
            solution.push(row_str);
        }
        result.push(solution);
        return;
    }
    for col in 0..n {
        if !can_place(board, row, col) {
            continue;
        }
        board[row][col] = 'Q';
        place(row + 1, board, result);
        board[row][col] = '.';
    }
}

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    let mut result = Vec::new();
    place(0, &mut vec![vec!['.'; n]; n], &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(n: i32, expected: &[&[&str]]) {
        let expected: Vec<Vec<String>> = expected
            .into_iter()
            .map(|solution| solution.into_iter().map(|s| s.to_string()).collect())
            .collect();
        assert_eq!(solve_n_queens(n), expected);
    }

    #[test]
    fn test_1() {
        check(
            4,
            &[
                &[".Q..", "...Q", "Q...", "..Q."],
                &["..Q.", "Q...", "...Q", ".Q.."],
            ],
        );
    }
    #[test]
    fn test_2() {
        check(1, &[&["Q"]]);
    }
}
