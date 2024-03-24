// https://leetcode.com/problems/find-the-duplicate-number/

pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    // Imagine node i has no value and nums[i] is the next pointer. We have node 0 pointing to node
    // nums[0], node nums[0] pointing to node nums[nums[0]], etc. The list has n+1 elements, but
    // each number is in the range 1..=n, so this won't be be out of bound.
    //
    // This list is guaranteed to contain a cycle and the beginning of the cycle is the number we
    // are looking for. The reason is: nums[i] serves as next pointer, so we are looking for
    // duplicated next pointer. That means we are looking for the node where there are multiple
    // paths to reach. In a linked list with cycle, the only node reachable from multiple different
    // nodes is the beginning of the cycle.
    let mut slow = nums[0];
    let mut fast = nums[nums[0] as usize];

    // Find a node in the cycle. This may not be the duplicated number
    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[fast as usize];
        fast = nums[fast as usize];
    }

    // Find the entrance to the cycle. This is the duplicated number.
    // See https://cs.stackexchange.com/a/90990 for explanation
    slow = 0;
    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[fast as usize];
    }

    slow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    }
    #[test]
    fn test_2() {
        assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }
    #[test]
    fn test_3() {
        assert_eq!(find_duplicate(vec![3, 3, 3, 3]), 3);
    }
    #[test]
    fn test_4() {
        assert_eq!(find_duplicate(vec![2, 2, 2, 3]), 2);
    }
    #[test]
    fn test_5() {
        assert_eq!(find_duplicate(vec![1, 2, 3, 3]), 3);
    }
}
