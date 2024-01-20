// https://leetcode.com/problems/guess-number-higher-or-lower/

fn guess(_: i32) -> i32 {
    6
}

pub fn guess_number(n: i32) -> i32 {
    let mut left = 1;
    let mut right = n;
    loop {
        let mid = left + (right - left) / 2;
        match guess(mid) {
            0 => return mid,
            -1 => right = mid - 1,
            1 => left = mid + 1,
            _ => unreachable!(),
        }
    }
}
