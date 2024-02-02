struct Solution;

impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let len = nums.len();
        let mut gap: HashMap<i32, usize> = HashMap::new();
        let mut position: HashMap<i32, usize> = HashMap::new();

        for idx in 0..2 * len {
            let num = nums[idx % len];
            position.entry(num)
                // 再次遇到 num, 更新最大间隔
                .and_modify(|prev| {
                    let this_gap = idx - *prev;
                    gap
                        .entry(num)
                        .and_modify(|prev_gap| {
                            if *prev_gap < this_gap { *prev_gap = this_gap; }
                        })
                        .or_insert(this_gap);

                    *prev = idx;
                })
                // 第一次遇上 num, 仅记录
                .or_insert(idx);
        }

        // (idx, val)
        let mut min = usize::MAX;
        for gap in gap.values() {
            if gap < &min {
                min = *gap;
            }
        }

        // println!("gap = {:#?}", gap);
        // println!("min gap = {}", min);

        // gap = 中间的数字数量+1
        // result = (中间的数字数量/2)向上取整 = (中间的数字数量+1)/2 = gap/2
        (min / 2) as i32
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("1 = {}", Solution::minimum_seconds(vec![8, 13, 3, 3]));
        println!("1 = {}", Solution::minimum_seconds(vec![1, 2, 1, 2]));
        println!("2 = {}", Solution::minimum_seconds(vec![2, 1, 3, 3, 2]));
        println!("0 = {}", Solution::minimum_seconds(vec![5, 5, 5, 5]));
    }
}