struct Solution;

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut result = vec![];

        for word in words {
            let segs = word.split(separator).collect::<Vec<_>>();
            for seg in segs {
                if !seg.is_empty() {
                    result.push(seg.to_string())
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        // let str = String::from(".a.b..");
        // let r = str.split(".").filter(|seg| !seg.is_empty()).collect::<Vec<_>>();

        println!("{:?}", Solution::split_words_by_separator(vec!["one.two.three".to_string(), "four.five".to_string(), "six".to_string()], '.'));
    }
}