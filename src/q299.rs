use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let chain = secret.chars().zip(guess.chars());

        let mut dict = HashMap::new();
        let mut a = 0;
        for (ch1, ch2) in chain.clone() {
            if ch1 == ch2 {
                a += 1;
            } else {
                *dict.entry(ch1).or_insert(0) += 1;
            }
        }

        if a == secret.len() {
            format!("{a}A0B")
        } else {
            let mut b = 0;

            for (ch1, ch2) in chain {
                if ch1 != ch2 {
                    if let Some(count) = dict.get_mut(&ch2) {
                        if *count > 0 {
                            b += 1;
                            *count -= 1;
                        }
                    }
                }
            }

            format!("{a}A{b}B")
        }
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        let str = "abc".to_string();
        let str2 = "def".to_string();

        let chain = str.chars().zip(str2.chars());
        println!("{:?}", chain);

        for (ch1, ch2) in chain {
            println!("{}, {}", ch1, ch2);
        }
    }
}