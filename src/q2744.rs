struct Solution;

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        use std::collections::HashSet;
        // required in leetcode's compiler
        use std::iter::FromIterator;

        let set: HashSet<&String> = HashSet::from_iter(words.iter());

        let mut count = 0;
        for word in &words {
            let reversed = word.chars().rev().collect::<String>();
            if word == &reversed { continue; }
            if set.contains(&reversed) { count += 1; }
        }

        count / 2
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        let words = vec!["cd", "ac", "dc", "ca", "zz"].iter().map(|v| v.to_string()).collect::<Vec<String>>();

        println!("2 = {}", Solution::maximum_number_of_string_pairs(words));
    }
}