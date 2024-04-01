struct Solution;

impl Solution {
    pub fn final_string(s: String) -> String {
        let mut queue = std::collections::VecDeque::new();
        let mut rev = false;

        for c in s.chars() {
            if c == 'i' {
                rev = !rev;
            } else {
                if rev {
                    queue.push_front(c);
                } else {
                    queue.push_back(c);
                }
            }
        }

        if rev { String::from_iter(queue.iter().rev()) } else { String::from_iter(queue) }
    }
}

#[test]
fn t() {
    println!("rtsng = {}", Solution::final_string("string".to_string()));
    println!("ponter = {}", Solution::final_string("poiinter".to_string()));
}