struct Solution;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();

        // build
        for ch1 in words1 {
            *map.entry(ch1).or_insert(0) += 1;
        }

        // consume
        for ch2 in words2 {
            if let Some(v) = map.get_mut(&ch2) {
                if v == &1 {
                    // 0 is counted
                    *v = 0;
                } else {
                    // -1 is dropped
                    *v = -1;
                }
            }
        }

        map.iter().fold(0, |mut count, item| {
            if item.1 == &0 {
                count += 1;
            }

            count
        })
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        let s1 = vec!["leetcode".to_string(), "is".to_string(), "amazing".to_string(), "as".to_string(), "is".to_string()];
        let s2 = vec!["amazing".to_string(), "leetcode".to_string(), "is".to_string()];

        println!("2 = {}", Solution::count_words(s1, s2));
    }
}