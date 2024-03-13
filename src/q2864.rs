struct Solution;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut zero = 0;
        let mut one = 0;
        for char in s.chars() {
            if char == '0' {
                zero += 1;
            } else {
                one += 1;
            }
        }

        let mut res = "".to_string();
        for _ in 0..one - 1 {
            res.push('1');
        }
        for _ in 0..zero {
            res.push('0');
        }
        res.push('1');

        res
    }
}