// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended

pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    events.sort_unstable();

    // If event A starts before event B, we should attend A first because after that we still have
    // time to attend B. However, that only applies to events in the future. If some events are
    // happening, the best course of action is to attend the one with earliest end time, start time
    // does not matter anymore as they are already started.

    // Track end time of ongoing events
    let mut ongoing_events: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut current_time = 0;
    let mut count = 0;
    let mut next_event_index = 0;
    while next_event_index < events.len() {
        // Remove ended events
        while let Some(&Reverse(end)) = ongoing_events.peek() {
            if end >= current_time {
                break;
            }
            ongoing_events.pop();
        }
        // Add newly started event
        while next_event_index < events.len() {
            if events[next_event_index][0] != current_time {
                break;
            }
            ongoing_events.push(Reverse(events[next_event_index][1]));
            next_event_index += 1;
        }
        match ongoing_events.pop() {
            Some(Reverse(_)) => {
                // Attend the one with nearest end time
                count += 1;
                current_time += 1;
            }
            None => {
                // No ongoing events, fast forward to the next future event
                if next_event_index < events.len() {
                    current_time = events[next_event_index][0];
                }
            }
        }
    }
    while let Some(Reverse(end)) = ongoing_events.pop() {
        if current_time <= end {
            current_time += 1;
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(
            max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]]),
            4
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            max_events(vec![
                vec![1, 4],
                vec![4, 4],
                vec![2, 2],
                vec![3, 4],
                vec![1, 1]
            ]),
            4
        );
    }
    #[test]
    fn test_4() {
        assert_eq!(
            max_events(vec![
                vec![1, 2],
                vec![1, 2],
                vec![3, 3],
                vec![1, 5],
                vec![1, 5],
            ]),
            5
        );
    }
    #[test]
    fn test_5() {
        assert_eq!(
            max_events(vec![
                vec![1, 5],
                vec![1, 5],
                vec![1, 5],
                vec![2, 3],
                vec![2, 3],
            ]),
            5
        );
    }
    #[test]
    fn test_6() {
        assert_eq!(
            max_events(vec![
                vec![1, 2],
                vec![1, 2],
                vec![1, 6],
                vec![1, 2],
                vec![1, 2],
            ]),
            3
        );
    }
}
