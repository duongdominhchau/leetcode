// https://leetcode.com/problems/task-scheduler/

pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    // Assuming that the number of unique tasks <= n + 1, idle time is required.
    // For example: with n=2 and 2 unique tasks we have something like this:
    //
    //     A B . A
    //
    // Note that there need to be n tasks between 2 executions of A, so counting
    // A into that we have a cycle of length n + 1
    //
    // In this case, only the task with largest frequency matters, as when its
    // last execution happens, everything else are already finished (we don't
    // lack time, we even have idle time). Therefore, in this case we need
    //
    //     num_max_freq + (maxFreq - 1) * (n + 1)
    //
    // cycles to finish all tasks.
    //
    // For the other case where number of unique tasks > n, we can't fit all
    // unique tasks into a cycle of length n + 1, so some will be left until
    // the next cycle. However, they need to run fewer times, so we should
    // eventually have time slots to fit them in.
    //
    // Consider the last round, at that point we have already executed
    // `max_freq - 1` rounds. Not counting the task with largest frequency,
    // we can run `n` other unique tasks each round, the total number of
    // slots available for schedule is `(max_freq - 1) * n`.
    // - If the sum of all the other tasks fits within these slot, we only
    // need to consider the most frequent task like before. We only need to
    // consider the sum because these tasks has frequency < max_freq and we
    // assumed the number of these tasks <= n, so even when all of them are
    // `max_freq - 1`, they still fit into the first `max_freq - 1` rounds.
    // - If the number of unique tasks > n, some tasks will not fit into the
    // round and need to wait until the next round. If there are so many tasks
    // to handle, we won't have any idle time, so it's just the sum of all
    // frequency. TODO: Proof that idle time is impossible in this case

    use std::collections::HashMap;

    let mut freq = HashMap::new();
    for ch in tasks.iter() {
        freq.entry(ch).and_modify(|c| *c += 1).or_insert(1);
    }

    let max_freq = freq.iter().map(|(_, &count)| count).max().unwrap();
    let num_max_freq = freq.values().filter(|&&count| count == max_freq).count() as i32;
    (tasks.len() as i32).max((max_freq - 1) * (n + 1) + num_max_freq)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2), 8);
    }
    #[test]
    fn test_2() {
        assert_eq!(least_interval(vec!['A', 'C', 'A', 'B', 'D', 'B'], 1), 6);
    }
    #[test]
    fn test_3() {
        assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 3), 10);
    }
    #[test]
    fn test_4() {
        assert_eq!(
            least_interval(
                vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
                1
            ),
            12
        );
    }
}
