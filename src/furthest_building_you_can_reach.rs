// https://leetcode.com/problems/furthest-building-you-can-reach/

pub fn furthest_building_ladders(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let n = heights.len();
    // Just use ladders all the way
    if ladders as usize >= n - 1 {
        return (n - 1) as i32;
    }
    // cost[i] is the cost to move into building i. We start at building 0, so cost[0] is always 0
    let mut cost = vec![0; n];
    for i in 1..n {
        cost[i] = 0.max(heights[i] - heights[i - 1]);
    }

    let mut current_pos = 0;
    // Value of the ladders used. If a ladder is usec to replace 20 bricks, its value is 20.
    let mut ladder_values = BinaryHeap::new();
    // First, we will use all ladders, then replace the ladder with lowest value by bricks. This
    // ensure the ladders are always being used to replace the highest number of bricks.
    for _ in 0..ladders {
        if current_pos == n - 1 {
            break;
        }
        current_pos += 1;
        ladder_values.push(Reverse(cost[current_pos]));
    }
    while current_pos < n - 1 {
        // Try moving to the next building. This is not the final decision yet
        current_pos += 1;
        // Replace lowest-value ladder with bricks
        ladder_values.push(Reverse(cost[current_pos]));
        let lowest_value = ladder_values.pop().unwrap().0;
        if bricks < lowest_value {
            // We don't have enough bricks to reach this position, moving back
            current_pos -= 1;
            break;
        }
        bricks -= lowest_value;
    }

    current_pos as i32
}
pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
    use std::collections::BinaryHeap;

    let n = heights.len();
    if ladders as usize >= n - 1 {
        return (n - 1) as i32;
    }

    // Why this works? Because to reach position k you must first reach position k-1.
    // If you used all your bricks to reach k-1. Whenever you use a ladder, you get some bricks
    // back. There are two possibilities:
    // - The bricks returned is enough (or more than enough) to get past building k+1, so this
    // is a good trade of ladder for bricks (ladder can get past **exactly** 1 building while the
    // amount of bricks returned can be used to get past **at least** 1 building
    // - The bricks returned is not enough to get past building k+1, in that case it does not
    // matter if we used ladder or bricks to go from k-1 to k.
    //
    // Therefore, once we run out of bricks, a ladder-to-brick trade is always optimal. We won't
    // get into a situation where that trade can yield higher value later.
    let mut bricks_used = BinaryHeap::new();
    for i in 1..n {
        if heights[i] <= heights[i - 1] {
            continue;
        }
        let diff = heights[i] - heights[i - 1];
        bricks -= diff;
        bricks_used.push(diff);
        while bricks < 0 {
            if ladders == 0 {
                return i as i32 - 1;
            }
            ladders -= 1;
            bricks += bricks_used.pop().unwrap();
        }
    }
    (n - 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1), 4);
    }
    #[test]
    fn test_2() {
        assert_eq!(
            furthest_building(vec![4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2),
            7
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(furthest_building(vec![14, 3, 19, 3], 17, 0), 3);
    }
    #[test]
    fn test_4() {
        assert_eq!(
            furthest_building(vec![1, 13, 1, 1, 13, 5, 11, 11], 10, 8),
            7
        );
    }
    #[test]
    fn test_5() {
        assert_eq!(furthest_building(vec![1, 5, 1, 2, 3, 4, 10000], 4, 1), 5);
    }
}
