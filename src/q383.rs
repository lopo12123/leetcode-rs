struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map: std::collections::HashMap<char, i32> = std::collections::HashMap::new();

        for char1 in magazine.chars() {
            map.insert(char1, map.get(&char1).unwrap_or(&0) + 1);
        }

        for char2 in ransom_note.chars() {
            let v = map.get(&char2).unwrap_or(&0) - 1;
            if v < 0 { return false; }
            map.insert(char2, v);
        }

        true
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("{}", Solution::can_construct("aa".to_string(), "aab".to_string()));
    }
}