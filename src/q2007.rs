use std::cmp::Ordering;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
        changed.sort_unstable();

        let mut original = vec![];

        let mut changed = changed.into_iter();
        let mut queue = VecDeque::new();
        queue.push_back(changed.next().unwrap());

        'task: loop {
            if queue.is_empty() {
                // 队列为空, 取下一项
                match changed.next() {
                    // 队列清空且处理完毕 => 是双倍数组
                    None => return original,
                    // 否则将下一项加入处理队列
                    Some(v) => queue.push_back(v),
                }
            } else {
                // 队列非空, 取队列第一项
                let candidate = queue.pop_front().unwrap();

                // 在队列中寻找 2v
                let target = candidate * 2;
                'find: while let Some(next) = changed.next() {
                    match next.cmp(&target) {
                        Ordering::Less => {
                            // 不够, 加入队列
                            queue.push_back(next);
                        }
                        Ordering::Equal => {
                            // 命中, 结束寻找
                            original.push(candidate);
                            continue 'task;
                        }
                        Ordering::Greater => {
                            // 超过, 直接结束 task
                            return vec![];
                        }
                    }
                }

                // 找不到就直接结束 task
                return vec![];
            }
        }
    }
}