struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { false } else {
            let mut xx = x;
            let mut rev = 0;

            while xx > 0 {
                rev = rev * 10 + xx % 10;
                xx /= 10;
            }

            x == rev
        }
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("{}", Solution::is_palindrome(121));
        println!("{}", Solution::is_palindrome(-121));
        println!("{}", Solution::is_palindrome(10));
    }
}