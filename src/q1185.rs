struct Solution;

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        fn is_leap_year(y: i32) -> bool {
            (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
        }

        // 1970-12-31 is Thursday, offset to Friday
        let weekdays = vec!["Thursday", "Friday", "Saturday", "Sunday", "Monday", "Tuesday", "Wednesday"];

        // 日
        let mut days = day;

        // 整年
        for y in 1971..year {
            days += if is_leap_year(y) { 366 } else { 365 }
        }

        // 整月
        let month_days = vec![31, if is_leap_year(year) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        for m in 0..(month - 1) as usize {
            days += month_days[m];
        }

        println!("{}", days);

        weekdays[(days % 7) as usize].to_string()
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("Friday {}", Solution::day_of_the_week(1, 1, 1971));  // days = 1
        println!("Wednesday {}", Solution::day_of_the_week(1, 3, 1972));  // days = 365+31+29+1 = 426
        println!("Monday {}", Solution::day_of_the_week(1, 1, 1973));  // days = 365+366+1 = 732
        println!("Saturday {}", Solution::day_of_the_week(31, 8, 2019));
        println!("Sunday {}", Solution::day_of_the_week(18, 7, 1999));
        println!("Sunday {}", Solution::day_of_the_week(15, 8, 1993));
    }
}