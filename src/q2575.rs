struct Solution;

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let m: i64 = m as i64;
        let mut re = vec![];

        let mut n: i64 = 0;
        for char in word.chars() {
            n = (n * 10 + char.to_digit(10).unwrap() as i64) % m;
            re.push(if n == 0 { 1 } else { 0 });
        }

        re
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        let r = Solution::divisibility_array("1000000000100000000030199999999610000000009".to_string(), 1000000000);
        println!("{:?}", r);
    }
}