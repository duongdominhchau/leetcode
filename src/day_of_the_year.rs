// https://leetcode.com/problems/day-of-the-year/

pub fn day_of_year(date: String) -> i32 {
    let v: Vec<i32> = date.split('-').map(|s| s.parse().unwrap()).collect();
    let (year, month, day) = (v[0], v[1], v[2]);
    let leap_year = year % 400 == 0 || year % 4 == 0 && year % 100 != 0;
    let days_in_month = vec![
        0,
        31,
        if leap_year { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    (1..month).map(|m| days_in_month[m as usize]).sum::<i32>() + day
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(day_of_year("2019-01-09".to_string()), 9);
    }
    #[test]
    fn test_2() {
        assert_eq!(day_of_year("2019-02-10".to_string()), 41);
    }
}
