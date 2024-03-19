struct NumArray {
    sum_now: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sum_now = vec![0; nums.len()];

        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            sum_now[i] = sum;
        }

        NumArray {
            sum_now
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
            self.sum_now[right as usize]
        } else {
            self.sum_now[right as usize] - self.sum_now[left as usize - 1]
        }
    }
}