struct Solution;

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut diffs = vec![];
        for i in 1..nums.len() {
            diffs.push(nums[i] - nums[i - 1]);
        }

        let mut begin_idx = None;
        let mut max = 0;
        diffs.iter().enumerate().fold(0, |prev, (idx, &curr)| {
            // 开始记录且连续 -- 直接跳过
            if begin_idx.is_some() && curr == 1 && prev == -1 || curr == -1 && prev == 1 {
                // noop
            } else {
                // 结算
                if let Some(begin) = begin_idx {
                    // println!("end at {idx}");
                    max = std::cmp::max(max, idx - begin + 1);
                    begin_idx = None;
                }

                // 标记新起点
                if curr == 1 {
                    // println!("begin at {idx}");
                    begin_idx = Some(idx);
                }
            }


            return curr;
        });

        // 结算
        if let Some(begin) = begin_idx {
            // println!("end at end");
            max = std::cmp::max(max, nums.len() - begin);
            begin_idx = None;
        }

        // println!("{:?}", diffs);

        if max == 0 { -1 } else { max as i32 }
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("-1 = {}", Solution::alternating_subarray(vec![5, 4, 3, 2, 1]));
        println!("4 = {}", Solution::alternating_subarray(vec![2, 3, 4, 3, 4]));
        println!("2 = {}", Solution::alternating_subarray(vec![4, 5, 6]));
        println!("4 = {}", Solution::alternating_subarray(vec![4, 5, 6, 4, 3, 5, 3, 4, 3, 4, 5, 1, 2]));
        println!("-1 = {}", Solution::alternating_subarray(vec![14, 30, 29, 49, 3, 23, 44, 21, 26, 52]));
        println!("2 = {}", Solution::alternating_subarray(vec![3, 3, 3, 2, 3, 3, 2, 1, 1]));
    }
}