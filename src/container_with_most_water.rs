// https://leetcode.com/problems/container-with-most-water

fn area(height: &[i32], a: usize, b: usize) -> i32 {
    (b - a) as i32 * height[a].min(height[b])
}

/// Idea: Move the shorter pole toward the other pole until it becomes the longer pole.
/// On the way, update the largest area we can get
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0_usize;
    let mut right = height.len() - 1;
    let mut best = area(&height, left, right);
    while left < right {
        if height[left] < height[right] {
            while left < right {
                left += 1;
                let new_area = area(&height, left, right);
                if new_area > best {
                    best = new_area;
                }
                if height[left] > height[right] {
                    break;
                }
            }
        } else {
            while left < right {
                right -= 1;
                let new_area = area(&height, left, right);
                if new_area > best {
                    best = new_area;
                }
                if height[left] < height[right] {
                    break;
                }
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
    #[test]
    fn test_2() {
        assert_eq!(max_area(vec![1, 1]), 1);
    }
    #[test]
    fn test_3() {
        assert_eq!(max_area(vec![1, 3, 2, 5, 25, 24, 5]), 24);
    }
    #[test]
    fn test_4() {
        assert_eq!(max_area(vec![1, 2, 3, 4, 5, 25, 24, 3, 4]), 24);
    }
}
