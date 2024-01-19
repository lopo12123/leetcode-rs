struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for char in s.chars() {
            let v = match char {
                '(' => 1,
                ')' => -1,
                '{' => 2,
                '}' => -2,
                '[' => 3,
                ']' => -3,
                _ => unreachable!()
            };

            if v > 0 { stack.push(v) } else {
                if stack.last().is_some_and(|top| top + &v == 0) {
                    stack.pop();
                } else {
                    return false;
                }
            }
        }


        stack.is_empty()
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("true = {}", Solution::is_valid("()".to_string()));
        println!("true = {}", Solution::is_valid("()[]{}".to_string()));
        println!("false = {}", Solution::is_valid("(]".to_string()));
    }
}