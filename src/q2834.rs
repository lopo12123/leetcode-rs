struct Solution;

impl Solution {
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        // 取 1 - (target~/2), 然后取 target - (target+(target~/2)-1)

        let n: i64 = n as i64;
        let target: i64 = target as i64;

        let base = 1e9 as i64 + 7;

        let n1 = target / 2;

        if n1 >= n {
            ((1 + n) * n / 2 % base) as i32
        } else {
            let sum1: i64 = (1 + n1) * n1 / 2 % base;

            let n2 = n - n1;
            let sum2 = (target + target + n2 - 1) * n2 / 2 % base;

            ((sum1 + sum2) % base) as i32
        }
    }
}