// https://leetcode.com/problems/bag-of-tokens/

pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
    let n = tokens.len();
    tokens.sort_unstable();

    let mut score = 0;
    // First, take all the smallest tokens first
    let mut i = 0;
    while i < n && power >= tokens[i] {
        score += 1;
        power -= tokens[i];
        i += 1;
    }
    // Need at least 1 score to play it face-down. The loop above stops when there is no other
    // token for us to take, so it is impossible to increase our score here.
    if score == 0 {
        return 0;
    }
    // Next, we consider if we can take a pair of tokens to increase power without losing score
    let mut left = i;
    let mut right = n - 1;
    while left < right {
        if power >= tokens[left] {
            score += 1;
            power -= tokens[left];
            left += 1;
            continue;
        }
        // Always take a pair to ensure our score does not decrease
        power -= tokens[left];
        left += 1;
        power += tokens[right];
        right -= 1;
    }
    // The loop above may stop when there is exactly 1 element between left and right.
    // In that case, that element is not checked yet, so we need to handle it here.
    if left == right && power >= tokens[left] {
        // power -= tokens[left];
        score += 1;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(bag_of_tokens_score(vec![100], 50), 0);
    }
    #[test]
    fn test_2() {
        assert_eq!(bag_of_tokens_score(vec![200, 100], 150), 1);
    }
    #[test]
    fn test_3() {
        assert_eq!(bag_of_tokens_score(vec![100, 200, 300, 400], 200), 2);
    }
    #[test]
    fn test_4() {
        assert_eq!(bag_of_tokens_score(Vec::new(), 200), 0);
    }
    #[test]
    fn test_5() {
        assert_eq!(bag_of_tokens_score(vec![71, 55, 82], 54), 0);
    }
    #[test]
    fn test_6() {
        assert_eq!(
            bag_of_tokens_score(vec![20, 61, 22, 34, 97, 90, 82, 3, 14, 25], 79),
            6
        );
    }
}
