// https://leetcode.com/problems/cheapest-flights-within-k-stops

pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    use std::collections::VecDeque;
    let (n, k, src, dst) = (n as usize, k as usize, src as usize, dst as usize);

    let mut adj = vec![vec![0i32; n]; n];
    for f in flights {
        let (from, to, price) = (f[0] as usize, f[1] as usize, f[2]);
        adj[from][to] = price;
    }

    // cheapest[i] = Lowest price we need to pay to go from `src` to `i`
    let mut cheapest = vec![i32::MAX; n];
    cheapest[src] = 0;
    // queue(node, path length so far)
    let mut queue = VecDeque::new();
    queue.push_back((src, 0));
    let mut i = 0;
    // Loop over k levels, stop early if there is nothing else to visit
    while i <= k && !queue.is_empty() {
        // Note: We use the same queue to traverse multiple levels, `queue.len()` will change while
        // the loop is running so we need to know beforehand how many nodes left in this level.
        // Here we rely on the fact that `queue.len()` will be copied into the range and thus will
        // not change.
        for _ in 0..queue.len() {
            if let Some((node, distance)) = queue.pop_front() {
                let (node, distance) = (node as usize, distance);
                // We don't use `visited` array as there may exist a better path through visited nodes
                // This should not cause an infinite loop because we have depth limit of `k`
                let neighbors = (0..n).filter(|&j| adj[node][j] > 0);
                for neighbor in neighbors {
                    let current_cost = distance + adj[node][neighbor];
                    if cheapest[neighbor] > current_cost {
                        cheapest[neighbor] = current_cost;
                        queue.push_back((neighbor, cheapest[neighbor]));
                    }
                }
            }
        }
        i += 1;
    }
    if cheapest[dst] != i32::MAX {
        cheapest[dst]
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            find_cheapest_price(
                4,
                vec![
                    vec![0, 1, 100],
                    vec![1, 2, 100],
                    vec![2, 0, 100],
                    vec![1, 3, 600],
                    vec![2, 3, 200]
                ],
                0,
                3,
                1
            ),
            700
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                1
            ),
            200
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                0
            ),
            500
        );
    }
    #[test]
    fn test_4() {
        assert_eq!(
            find_cheapest_price(
                5,
                vec![
                    vec![4, 1, 1],
                    vec![1, 2, 3],
                    vec![0, 3, 2],
                    vec![0, 4, 10],
                    vec![3, 1, 1],
                    vec![1, 4, 3]
                ],
                2,
                1,
                1
            ),
            -1
        );
    }
    #[test]
    fn test_5() {
        assert_eq!(
            find_cheapest_price(
                4,
                vec![vec![0, 1, 1], vec![0, 2, 5], vec![1, 2, 1], vec![2, 3, 1]],
                0,
                3,
                1
            ),
            6
        );
    }
}
