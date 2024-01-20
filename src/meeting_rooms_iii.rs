// https://leetcode.com/problems/meeting-rooms-iii

pub fn most_booked_linear_search(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;

    // Sort by start time
    meetings.sort_unstable();

    // Can use a binary heap instead, but I'm too lazy to implement that
    let mut next_free = vec![0_i64; n];
    let mut meetings_held = vec![0; n];

    for meeting in meetings {
        let (start, end) = (meeting[0] as i64, meeting[1] as i64);
        let duration = end - start;
        let next_room = {
            let mut next_room = usize::MAX;
            let mut min_next_free_index = 0;
            for (index, &next) in next_free.iter().enumerate() {
                // Found a free room to start the meeting on-time
                if next <= start {
                    next_room = index;
                    break;
                }
                // The meeting needs to be delayed, find the earliest free room
                if next < next_free[min_next_free_index] {
                    min_next_free_index = index;
                }
            }
            if next_room != usize::MAX {
                next_room
            } else {
                min_next_free_index
            }
        };
        meetings_held[next_room] += 1;
        next_free[next_room] = next_free[next_room].max(start) + duration;
    }

    meetings_held
        .into_iter()
        .enumerate()
        .max_by_key(|&(index, count)| (count, std::cmp::Reverse(index)))
        .map(|(index, _count)| index as i32)
        .unwrap()
}

pub fn most_booked_min_heap(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = n as usize;

    // Sort by start time, the problem statement guaranteed that start time is unique
    meetings.sort_unstable();

    let mut meetings_held = vec![0; n];
    let mut free_rooms: BinaryHeap<Reverse<usize>> = (0..n).map(Reverse).collect();
    // occupied_rooms contains (free_at, room) pairs
    let mut occupied_rooms: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();

    for meeting in meetings {
        let (start, end) = (meeting[0] as i64, meeting[1] as i64);
        let duration = end - start;

        // Reclaim rooms with finished meeting
        loop {
            let occupied_room = occupied_rooms.peek();
            if occupied_room.is_none() || occupied_room.unwrap().0 .0 > start {
                break;
            }
            let occupied_room = occupied_rooms.pop().unwrap().0 .1;
            free_rooms.push(Reverse(occupied_room));
        }

        match free_rooms.pop() {
            Some(Reverse(room)) => {
                // Have free room, use it
                meetings_held[room] += 1;
                occupied_rooms.push(Reverse((end, room)));
            }
            None => {
                // No free room, need to delay the meeting and wait for the room
                let room = occupied_rooms.pop().unwrap().0;
                meetings_held[room.1] += 1;
                occupied_rooms.push(Reverse((room.0 + duration, room.1)));
            }
        }
    }

    meetings_held
        .into_iter()
        .enumerate()
        .max_by_key(|&(index, count)| (count, Reverse(index)))
        .unwrap()
        .0 as i32
}

pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
    most_booked_linear_search(n, meetings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]]),
            0
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            most_booked(
                3,
                vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]]
            ),
            1
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            most_booked(
                4,
                vec![
                    vec![18, 19],
                    vec![3, 12],
                    vec![17, 19],
                    vec![2, 13],
                    vec![7, 10]
                ]
            ),
            0
        );
    }
    #[test]
    fn test_4() {
        let file = std::fs::File::open("meeting_rooms_3_test_case_80.json").unwrap();
        let reader = std::io::BufReader::new(file);
        let input: Vec<Vec<i32>> = serde_json::from_reader(reader).unwrap();
        assert_eq!(most_booked(4, input), 1);
    }
}
