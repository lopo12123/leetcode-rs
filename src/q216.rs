struct Solution;

impl Solution {
    fn pick_or_skip(mut mask: i32, k: i32, n: i32) -> Option<Vec<i32>> {
        let mut nums = vec![];
        let mut sum = 0;
        for i in 1..=9 {
            if mask & 1 == 1 {
                nums.push(i);
                sum += i;
            }
            mask >>= 1;

            if mask == 0 || sum > n {
                break;
            }
        }

        if sum == n && nums.len() == k as usize {
            Some(nums)
        } else {
            None
        }
    }

    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let max = 2i32.pow(9);
        let mut result = vec![];
        for mask in 1..=max {
            if let Some(combine) = Self::pick_or_skip(mask, k, n) {
                result.push(combine);
            }
        }
        result
    }
}