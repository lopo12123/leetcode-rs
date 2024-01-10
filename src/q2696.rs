struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack = vec![];

        for ch in s.chars() {
            match stack.last() {
                Some(top) => {
                    if ch == 'B' && top == &'A' || ch == 'D' && top == &'C' {
                        stack.pop();
                    } else {
                        stack.push(ch);
                    }
                }
                None => stack.push(ch)
            }
        }

        stack.len() as i32
    }
}

#[cfg(test)]
mod unit_test {
    use crate::q2696::Solution;

    #[test]
    fn t() {
        println!("2 = {}", Solution::min_length("ABFCACDB".to_string()));
        println!("5 = {}", Solution::min_length("ACBBD".to_string()));
    }
}