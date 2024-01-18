struct Solution;

impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        // 1. sort
        let mut sorted = beans;
        sorted.sort();

        // 2. find
        let len = sorted.len();
        // (index, S)
        let mut max = (0usize, 0i64);
        for (x, y) in sorted.iter().enumerate() {
            let s_i = *y as i64 * (len - x) as i64;
            if s_i > max.1 {
                max = (x, s_i);
            }
        }

        // println!("{:?}", max);

        // 3. calculate
        let threshold = sorted[max.0] as i64;
        // println!("{:?}", threshold);
        sorted.iter().fold(0i64, |mut count, this| {
            let this = *this as i64;
            count += if this < threshold { this } else { this - threshold };
            // println!("count = {count}");
            count
        })
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("4 = {}", Solution::minimum_removal(vec![4, 1, 6, 5]));
        println!("7 = {}", Solution::minimum_removal(vec![2, 10, 3, 2]));
        println!("4133 = {}", Solution::minimum_removal(vec![4, 5, 1, 23, 5, 6, 17213, 13124]));
    }
}