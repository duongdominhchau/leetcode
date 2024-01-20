// https://leetcode.com/problems/assign-cookies

pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    g.sort_unstable();
    s.sort_unstable();
    let mut k = 0;
    let mut total = 0;
    for &greed in g.iter() {
        // Skip smaller cookies, they are useless now
        while k < s.len() && s[k] < greed {
            k += 1;
        }
        if k == s.len() {
            break;
        }
        // Take the smallest valid cookie
        k += 1;
        total += 1;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_content_children(vec![1, 2, 3], vec![1, 1]), 1);
    }
}
