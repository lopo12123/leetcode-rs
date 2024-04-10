struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            nums[0]
        } else if len == 2 {
            nums[0].max(nums[1])
        } else {
            let mut dp = vec![nums[0], nums[0].max(nums[1])];

            for i in 2..len {
                dp.push(dp[i - 1].max(dp[i - 2] + nums[i]))
            }

            dp[len - 1]
        }
    }
}

#[test]
fn t() {
    Solution::rob(vec![2, 1, 1, 2]);
}