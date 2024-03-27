struct Solution;

impl Solution {
    pub fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
        const MOD_BASE: i32 = 1e9 as i32 + 7;
        // 根据左边界排序
        ranges.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        // 断点数
        let mut gaps = 0;

        // 取第一段的右边界作为游标起点
        let mut cursor = ranges[0][1];

        for i in 1..ranges.len() {
            let range = &ranges[i];

            if range[0] > cursor {
                // 如果没有交集, 断点加1并以此段为下一次的起点
                gaps += 1;
                cursor = range[1];
            } else {
                // 如果有交集, right 移动到当前最右侧的点
                cursor = cursor.max(range[1]);
            }
        }

        // 根据 gaps 计算结果
        let mut re = 2;
        for _ in 0..gaps {
            re = (re * 2) % MOD_BASE;
        }

        re
    }
}

#[test]
fn t() {
    println!("{}", i32::MAX);
    println!("{}", 1e9 + 7f64);
}