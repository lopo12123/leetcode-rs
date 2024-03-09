use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

struct Solution;

// 表示某一种选择 (diff值, 最后一项的下标)
struct Pick(i64, usize);

impl Eq for Pick {}

impl PartialEq<Self> for Pick {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for Pick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd<Self> for Pick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        // 所有正数之和为子序列的最大和
        let mut max_sum = 0i64;
        for &num in &nums {
            if num > 0 {
                max_sum += num as i64;
            }
        }

        // 1. 如果k为1, 直接返回最大和
        if k == 1 {
            return max_sum;
        }

        // 全部转换为正数用于减去, 并按照从小到大排序
        let mut candidates = nums.iter().map(|&x| x.abs()).collect::<Vec<i32>>();
        candidates.sort_unstable();

        // 使用小根堆用于存储所有的后续选择
        let mut choices: BinaryHeap<Reverse<Pick>> = BinaryHeap::new();
        // 第一种选择为减去1个绝对值最小的数
        choices.push(Reverse(Pick(candidates[0] as i64, 0)));

        // 标识下一次取出为第priority大 (max_sum 为第1大)
        let mut priority = 2;

        while priority < k {
            let Pick(diff, last_idx) = choices.pop().unwrap().0;

            // 还有剩余的数字, 继续构造选择
            if last_idx < candidates.len() - 1 {
                let last = candidates[last_idx];
                let next = candidates[last_idx + 1];
                // 选择1: 替换最后一项
                choices.push(Reverse(Pick(diff + next as i64 - last as i64, last_idx + 1)));
                // 选择2: 直接新增一项
                choices.push(Reverse(Pick(diff + next as i64, last_idx + 1)));
            }

            priority += 1;
        }

        // 下一种即为最终结果
        max_sum - choices.pop().unwrap().0.0
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        // let r = Solution::k_sum(vec![2, 4, -2], 5);  // 2
        // let r = Solution::k_sum(vec![1, -2, 3, 4, -10, 12], 16);  // 10
        let r = Solution::k_sum(vec![1], 1);
        println!("{}", r);
    }
}