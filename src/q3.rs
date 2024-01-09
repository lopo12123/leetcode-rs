struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut cache = std::collections::HashMap::new();

        let mut len = 0;

        // i=0 时 len=1, 所以需要tail初始为-1
        let mut tail = -1;

        for (i, ch) in s.chars().enumerate() {
            if cache.contains_key(&ch) {
                tail = (*cache.get(&ch).unwrap() as i32).max(tail);
            }

            len = std::cmp::max(len, i as i32 - tail);

            cache.insert(ch, i);

            // println!("i={i}, tail={tail}, ch={}, cache={:?}", &ch, &cache);
        }

        len
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("2 = {}", Solution::length_of_longest_substring("abba".to_string()));
        println!("2 = {}", Solution::length_of_longest_substring("aba".to_string()));
        println!("1 = {}", Solution::length_of_longest_substring("a".to_string()));
        println!("1 = {}", Solution::length_of_longest_substring("aa".to_string()));
        println!("3 = {}", Solution::length_of_longest_substring("abc".to_string()));
        println!("3 = {}", Solution::length_of_longest_substring("abcabcbb".to_string()));
        println!("1 = {}", Solution::length_of_longest_substring("bbbbb".to_string()));
        println!("3 = {}", Solution::length_of_longest_substring("pwwkew".to_string()));
    }
}