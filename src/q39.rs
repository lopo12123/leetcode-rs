use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();

        // 确保每次执行结束后 state 恢复原样, 用于后续的遍历
        fn find(result: &mut Vec<Vec<i32>>, candidates: &Vec<i32>, stats: &mut Vec<i32>, need: i32, ptr: usize) {
            // 没了
            if ptr >= candidates.len() {
                return;
            }

            let next = candidates[ptr];

            match need.cmp(&next) {
                // 无法满足, 直接返回
                Ordering::Less => return,
                // 刚好满足, 记录并结束
                Ordering::Equal => {
                    stats.push(next);
                    result.push(stats.clone());
                    stats.pop();
                }
                // 可以选择, 分类继续
                Ordering::Greater => {
                    // 选用 next
                    stats.push(next);
                    find(result, candidates, stats, need - next, ptr);

                    // 不用 next
                    stats.pop();
                    find(result, candidates, stats, need, ptr + 1);
                }
            }
        }

        let mut result = vec![];
        find(&mut result, &candidates, &mut vec![], target, 0);
        result
    }
}