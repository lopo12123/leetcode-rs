struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        use std::collections::HashMap;

        let len1 = haystack.len();
        let len2 = needle.len();
        if len1 < len2 { return -1; }
        let max_p = len1 - len2;

        let haystack_code = &haystack[..];
        let needle_code = needle.as_bytes();

        let mut offsets = HashMap::new();
        for (dis, ch) in needle_code.iter().rev().enumerate() {
            offsets.entry(*ch).or_insert(dis + 1);
        }

        let mut ptr = 0;
        loop {
            if &haystack[ptr..ptr + needle.len()] == &needle {
                return ptr as i32;
            }

            // 检查 待匹配字符串的后一位 (不是 ptr 的后一位)
            match haystack_code.get(ptr + len2..ptr + len2 + 1) {
                Some(ch) => {
                    let jump = *offsets.get(&ch.as_bytes()[0]).unwrap_or(&(len2 + 1));
                    ptr += jump;

                    if ptr > max_p { break; }
                }
                None => break
            }
        }

        -1
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("4 = {}", Solution::str_str("mississippi".to_string(), "issip".to_string()));
        println!("-1 = {}", Solution::str_str("aaaaa".to_string(), "bba".to_string()));
        println!("0 = {}", Solution::str_str("sadbutsad".to_string(), "sad".to_string()));
        println!("-1 = {}", Solution::str_str("leetcode".to_string(), "leeto".to_string()));
    }
}