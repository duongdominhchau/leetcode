// https://leetcode.com/problems/binary-watch/

fn time_parts(i: u32, n: u32) -> Option<(String, String)> {
    let hour = (i & 0b1111_000_000) >> 6;
    let minute = i & 0b0000_111_111;
    let ones_count_matches = hour.count_ones() + minute.count_ones() == n as u32;
    if ones_count_matches && hour < 12 && minute < 60 {
        Some((hour.to_string(), minute.to_string()))
    } else {
        None
    }
}
pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let turned_on = turned_on as u32;
    let mut result = Vec::new();

    // We have 4 lights for hour and 6 lights for minute. Try spending lights on the hour part
    // first, then use the rest for the minute part.
    //
    // We can use 4 bits to represent the hour part and 6 more bits for the minute part
    for i in (0u32..(2 << 10)).filter(|i| i.count_ones() == turned_on) {
        if let Some((hour, minute)) = time_parts(i, turned_on) {
            result.push(format!("{hour}:{minute:0>2}"));
        }
    }

    result.sort_unstable();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(n: i32, expected: Vec<&str>) {
        let expected: Vec<String> = expected.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(read_binary_watch(n), expected);
    }

    #[test]
    fn test_1() {
        check(
            1,
            vec![
                "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
            ],
        );
    }
    #[test]
    fn test_2() {
        check(9, vec![]);
    }
}
