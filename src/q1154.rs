struct Solution;

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        fn is_leap_year(y: i32) -> bool {
            y % 4 == 0 && y % 100 != 0 || y % 400 == 0
        }

        let parts: Vec<i32> = date.split('-').map(|s| s.parse::<i32>().unwrap()).collect();

        let days = [31, if is_leap_year(parts[0]) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        days.iter().take(parts[1] as usize - 1).sum::<i32>() + parts[2]
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("{}", Solution::day_of_year("2019-01-09".to_string()));
        println!("{}", Solution::day_of_year("2019-02-10".to_string()));
    }
}