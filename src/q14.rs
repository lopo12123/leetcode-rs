struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut max_len = 0;
        let matrix = strs.iter().map(|s| {
            if s.len() > max_len { max_len = s.len(); }
            s.as_bytes()
        }).collect::<Vec<_>>();

        let mut prefix = vec![];

        'forward: for offset in 0..max_len {
            match matrix[0].get(offset) {
                Some(char) => {
                    'scan: for idx in 1..strs.len() {
                        if matrix[idx].get(offset).is_some_and(|code| code == char) {
                            continue;
                        }

                        break 'forward;
                    }

                    prefix.push(*char);
                }
                None => break 'forward
            }
        }

        String::from_utf8(prefix).unwrap()
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("'fl' = '{}'", Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]));
        println!("'' = '{}'", Solution::longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]));
    }
}