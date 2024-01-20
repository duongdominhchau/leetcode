// https://leetcode.com/problems/combinations/

pub fn combine_bitwise(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    // Use 20-bit int to represent selection, starts with k one to the right,
    // ends with k ones to the left
    let start: i32 = (1 << k) - 1;
    let end = start << (n - k);
    for b in (start..=end).filter(|&x| x.count_ones() as i32 == k) {
        let mut v = Vec::new();
        for i in 0..20 {
            if b & (1 << i) != 0 {
                v.push(i + 1);
            }
        }
        result.push(v);
    }
    result
}
pub fn combine_complex(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    // We can represent combination as a list of digits in dynamic bases
    let mut combination = Vec::new();
    for i in 0..k {
        combination.push(i);
    }

    // Increase it until overflow and we will get all combinations. The last combination has n as
    // its last number, and the combination has length k, so its first number will be n-k+1
    while combination[0] < n - k + 1 {
        result.push(combination.iter().map(|x| x + 1).collect());
        // +1 to the combination while maintaining the sorted state
        let mut carry = 1;
        let mut last_changed_digit = k as usize;
        for i in (0..k as usize).rev() {
            combination[i] += carry;
            // At index k - 1 (last position), the largest number allowed is n - 1. At index i, the
            // largest number allowed is i + (n - k), so the base is i + (n - k) + 1
            let base = i as i32 + (n - k) + 1;
            carry = combination[i] / base;
            combination[i] %= base;
            last_changed_digit = i;
            if carry == 0 {
                break;
            }
        }
        // Restore sorted state
        for i in (last_changed_digit + 1)..k as usize {
            combination[i] = combination[i - 1] + 1;
        }
        // Overflown, we are done
        if carry != 0 {
            break;
        }
    }

    result
}

fn place(i: i32, start: i32, n: i32, k: i32, choices: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if i == k {
        result.push(choices.clone());
        return;
    }
    // After choosing a number for index i, we have k-1-i positions left to fill
    // So the range we need to traverse is start..=(n - remaining_cells)
    //
    // We don't really need to do this though, if start > n we will get an empty range and thus no
    // recursive call is made. Because of that, if we choose a value too large for a position,
    // latter positions will be skipped safely
    for j in start..=n {
        choices[i as usize] = j;
        place(i + 1, j + 1, n, k, choices, result);
    }
}
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut choices = vec![0; k as usize];
    place(0, 1, n, k, &mut choices, &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(n: i32, k: i32, mut expected: Vec<Vec<i32>>) {
        let mut actual = combine(n, k);
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        check(
            4,
            2,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ],
        );
    }
    #[test]
    fn test_2() {
        check(1, 1, vec![vec![1]]);
    }
    #[test]
    fn test_3() {
        check(3, 3, vec![vec![1, 2, 3]]);
    }
    #[test]
    fn test_4() {
        check(
            20,
            2,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![1, 5],
                vec![1, 6],
                vec![1, 7],
                vec![1, 8],
                vec![1, 9],
                vec![1, 10],
                vec![1, 11],
                vec![1, 12],
                vec![1, 13],
                vec![1, 14],
                vec![1, 15],
                vec![1, 16],
                vec![1, 17],
                vec![1, 18],
                vec![1, 19],
                vec![1, 20],
                vec![2, 3],
                vec![2, 4],
                vec![2, 5],
                vec![2, 6],
                vec![2, 7],
                vec![2, 8],
                vec![2, 9],
                vec![2, 10],
                vec![2, 11],
                vec![2, 12],
                vec![2, 13],
                vec![2, 14],
                vec![2, 15],
                vec![2, 16],
                vec![2, 17],
                vec![2, 18],
                vec![2, 19],
                vec![2, 20],
                vec![3, 4],
                vec![3, 5],
                vec![3, 6],
                vec![3, 7],
                vec![3, 8],
                vec![3, 9],
                vec![3, 10],
                vec![3, 11],
                vec![3, 12],
                vec![3, 13],
                vec![3, 14],
                vec![3, 15],
                vec![3, 16],
                vec![3, 17],
                vec![3, 18],
                vec![3, 19],
                vec![3, 20],
                vec![4, 5],
                vec![4, 6],
                vec![4, 7],
                vec![4, 8],
                vec![4, 9],
                vec![4, 10],
                vec![4, 11],
                vec![4, 12],
                vec![4, 13],
                vec![4, 14],
                vec![4, 15],
                vec![4, 16],
                vec![4, 17],
                vec![4, 18],
                vec![4, 19],
                vec![4, 20],
                vec![5, 6],
                vec![5, 7],
                vec![5, 8],
                vec![5, 9],
                vec![5, 10],
                vec![5, 11],
                vec![5, 12],
                vec![5, 13],
                vec![5, 14],
                vec![5, 15],
                vec![5, 16],
                vec![5, 17],
                vec![5, 18],
                vec![5, 19],
                vec![5, 20],
                vec![6, 7],
                vec![6, 8],
                vec![6, 9],
                vec![6, 10],
                vec![6, 11],
                vec![6, 12],
                vec![6, 13],
                vec![6, 14],
                vec![6, 15],
                vec![6, 16],
                vec![6, 17],
                vec![6, 18],
                vec![6, 19],
                vec![6, 20],
                vec![7, 8],
                vec![7, 9],
                vec![7, 10],
                vec![7, 11],
                vec![7, 12],
                vec![7, 13],
                vec![7, 14],
                vec![7, 15],
                vec![7, 16],
                vec![7, 17],
                vec![7, 18],
                vec![7, 19],
                vec![7, 20],
                vec![8, 9],
                vec![8, 10],
                vec![8, 11],
                vec![8, 12],
                vec![8, 13],
                vec![8, 14],
                vec![8, 15],
                vec![8, 16],
                vec![8, 17],
                vec![8, 18],
                vec![8, 19],
                vec![8, 20],
                vec![9, 10],
                vec![9, 11],
                vec![9, 12],
                vec![9, 13],
                vec![9, 14],
                vec![9, 15],
                vec![9, 16],
                vec![9, 17],
                vec![9, 18],
                vec![9, 19],
                vec![9, 20],
                vec![10, 11],
                vec![10, 12],
                vec![10, 13],
                vec![10, 14],
                vec![10, 15],
                vec![10, 16],
                vec![10, 17],
                vec![10, 18],
                vec![10, 19],
                vec![10, 20],
                vec![11, 12],
                vec![11, 13],
                vec![11, 14],
                vec![11, 15],
                vec![11, 16],
                vec![11, 17],
                vec![11, 18],
                vec![11, 19],
                vec![11, 20],
                vec![12, 13],
                vec![12, 14],
                vec![12, 15],
                vec![12, 16],
                vec![12, 17],
                vec![12, 18],
                vec![12, 19],
                vec![12, 20],
                vec![13, 14],
                vec![13, 15],
                vec![13, 16],
                vec![13, 17],
                vec![13, 18],
                vec![13, 19],
                vec![13, 20],
                vec![14, 15],
                vec![14, 16],
                vec![14, 17],
                vec![14, 18],
                vec![14, 19],
                vec![14, 20],
                vec![15, 16],
                vec![15, 17],
                vec![15, 18],
                vec![15, 19],
                vec![15, 20],
                vec![16, 17],
                vec![16, 18],
                vec![16, 19],
                vec![16, 20],
                vec![17, 18],
                vec![17, 19],
                vec![17, 20],
                vec![18, 19],
                vec![18, 20],
                vec![19, 20],
            ],
        );
    }
}
