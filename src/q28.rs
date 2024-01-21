struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map_or(-1, |v| v as i32)
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("0 = {}", Solution::str_str("sadbutsad".to_string(), "sad".to_string()));
        println!("-1 = {}", Solution::str_str("leetcode".to_string(), "leeto".to_string()));
    }
}