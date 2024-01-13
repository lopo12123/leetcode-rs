
struct Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        use std::collections::HashMap;

        // 统计字符个数
        let materials = s.into_bytes().into_iter().fold(HashMap::new(), |mut all, ch| {
            *all.entry(ch).or_insert(0) += 1;
            all
        });

        // 倒序排序
        let mut chars = materials.into_iter().collect::<Vec<(u8, i32)>>();
        chars.sort_by_key(|v| 200 - v.0);

        let mut result = vec![];
        let mut p_own = 0;
        'build: while p_own < chars.len() {
            // 当前字符情况
            let ch = chars[p_own];

            // 提前被借完, 直接跳过
            if ch.1 == 0 {
                // 处理下一个字符
                p_own += 1;
                continue;
            }

            if ch.1 <= repeat_limit {
                // 1. 未超出限制, 直接拼接
                result.extend(vec![ch.0; ch.1 as usize]);
            } else {
                // 2. 超出限制, 借用后续字符作为隔断
                // number of this char
                let mut ch_count = ch.1;
                // where to borrow
                let mut p_rent = p_own + 1;
                while ch_count > repeat_limit {
                    // 2-1. 插入最大量
                    result.extend(vec![ch.0; repeat_limit as usize]);
                    ch_count -= repeat_limit;

                    // 2-2. 借取
                    if p_rent < chars.len() {
                        // 借取成功, 插入
                        chars[p_rent].1 -= 1;
                        result.push(chars[p_rent].0);

                        // 借完, 下一家
                        if chars[p_rent].1 == 0 {
                            p_rent += 1;
                        }
                    } else {
                        // 借取失败, 直接退出构造
                        break 'build;
                    }
                }
                result.extend(vec![ch.0; ch_count as usize]);
            }

            // 处理下一个字符
            p_own += 1;
        }

        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("{}", Solution::repeat_limited_string("cczazcc".to_string(), 3));
    }
}