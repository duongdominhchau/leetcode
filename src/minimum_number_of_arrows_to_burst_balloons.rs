// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/

pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_unstable();
    let mut count = 0;
    let mut i = 0;
    while i < points.len() {
        // Shoot down current balloon
        let (mut start, mut end) = (points[i][0], points[i][1]);
        count += 1;
        i += 1;
        // If other balloons intersect with current one, they will be bursted as well.
        // However, the more balloons we want to burst in a single shot, the narrower
        // range of `x` we can choose
        while i < points.len() && start <= points[i][0] && points[i][0] <= end {
            start = points[i][0];
            end = end.min(points[i][1]);
            i += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &[[i32; 2]], expected: i32) {
        let input: Vec<Vec<i32>> = input
            .into_iter()
            .map(|&p| p.into_iter().collect())
            .collect();
        assert_eq!(find_min_arrow_shots(input), expected);
    }

    #[test]
    fn test_1() {
        check(&[[10, 16], [2, 8], [1, 6], [7, 12]], 2);
    }
    #[test]
    fn test_2() {
        check(&[[1, 2], [3, 4], [5, 6], [7, 8]], 4);
    }
    #[test]
    fn test_3() {
        check(&[[1, 2], [2, 3], [3, 4], [4, 5]], 2);
    }
    #[test]
    fn test_4() {
        check(
            &[
                [3, 9],
                [7, 12],
                [3, 8],
                [6, 8],
                [9, 10],
                [2, 9],
                [0, 9],
                [3, 9],
                [0, 6],
                [2, 8],
            ],
            2,
        );
    }
}
