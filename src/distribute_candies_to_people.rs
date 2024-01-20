// https://leetcode.com/problems/distribute-candies-to-people/

pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
    let n = num_people as usize;
    let mut result = vec![0; n];
    let mut i = 0;
    let mut k = 1;
    while candies > 0 {
        result[i] += k.min(candies);
        i = (i + 1) % n;
        candies -= k;
        k += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(distribute_candies(7, 4), vec![1, 2, 3, 1]);
    }
    #[test]
    fn test_2() {
        assert_eq!(distribute_candies(10, 3), vec![5, 2, 3]);
    }
}
