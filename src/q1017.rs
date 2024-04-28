use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn base_neg2(mut n: i32) -> String {
        if n == 0 {
            return "0".to_string();
        }

        let mut code: VecDeque<char> = VecDeque::new();

        while n != 0 {
            let b = n % 2 != 0;  // n % 2 == 1 or -1
            code.push_front(if b { '1' } else { '0' });
            if b {
                n -= 1;
            }
            n /= -2;
        }

        String::from_iter(code)
    }
}