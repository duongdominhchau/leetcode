// https://leetcode.com/problems/valid-perfect-square

pub fn is_perfect_square(num: i32) -> bool {
    if num == 1 {
        return true;
    }
    for i in 2..num {
        let square = i * i;
        if square == num {
            return true;
        }
        if square > num {
            break;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_perfect_square(16), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(is_perfect_square(14), false);
    }
    #[test]
    fn test_3() {
        assert_eq!(is_perfect_square(1), true);
    }
}
