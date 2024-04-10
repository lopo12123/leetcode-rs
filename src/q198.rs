struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            nums[0]
        } else if len == 2 {
            nums[0].max(nums[1])
        } else {
            // (even, odd)
            let mut dp = (nums[0], nums[0].max(nums[1]));

            for i in 2..len {
                if i % 2 == 0 {
                    dp.0 = dp.1.max(dp.0 + nums[i]);
                } else {
                    dp.1 = dp.0.max(dp.1 + nums[i]);
                }
            }
            dp.0.max(dp.1)
        }
    }
}

#[test]
fn t() {
    Solution::rob(vec![2, 1, 1, 2]);
}