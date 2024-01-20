#[derive(Debug)]
pub enum Direction {
    Down,
    Upright,
}

fn allocate_lines(num_rows: usize) -> Vec<String> {
    let mut lines = Vec::new();
    lines.reserve_exact(num_rows);
    for _ in 0..num_rows {
        lines.push(String::new());
    }
    lines
}

pub fn convert(s: String, num_rows: i32) -> String {
    // Need to handle this special case as we use usize for tracking current row
    // Subtracting 1 from 0 will overflow
    if num_rows == 1 {
        return s;
    }
    let num_rows = num_rows as usize;
    let mut direction = Direction::Down;
    // We define position of the first character to be (0,0)
    let mut row = 0;
    let mut lines = allocate_lines(num_rows);
    for ch in s.chars() {
        match direction {
            Direction::Down => {
                lines[row].push(ch);
                if row + 1 < num_rows {
                    row += 1;
                } else {
                    row -= 1;
                    direction = Direction::Upright;
                }
            }
            Direction::Upright => {
                // We don't care about whitespace, so this is not needed. However,
                // it is useful for debugging purpose, so I will leave the code here
                // for i in 0..row {
                //     lines[i].push(' ');
                // }
                lines[row].push(ch);
                if row > 0 {
                    row -= 1;
                } else {
                    row += 1;
                    direction = Direction::Down;
                }
            }
        }
    }
    lines.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(convert("A".to_string(), 1), "A".to_string());
    }
}
