struct Solution;

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut forward = vec![51; len];
        let mut backward = vec![51; len];

        let mut min = 51;
        for i in 0..len {
            forward[i] = min;
            min = min.min(nums[i]);
        }
        min = 51;
        for i in (0..len).rev() {
            backward[i] = min;
            min = min.min(nums[i]);
        }

        min = 151;
        for i in 1..len - 1 {
            if nums[i] > forward[i] && nums[i] > backward[i] {
                min = min.min(forward[i] + nums[i] + backward[i]);
            }
        }

        if min == 151 { -1 } else { min }
    }
}