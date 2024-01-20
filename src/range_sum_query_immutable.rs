struct NumArray {
    sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sum = nums;
        for i in 1..sum.len() {
            sum[i] += sum[i - 1];
        }
        Self { sum }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let (left, right) = (left as usize, right as usize);
        if left == 0 {
            self.sum[right]
        } else {
            self.sum[right] - self.sum[left - 1]
        }
    }
}
