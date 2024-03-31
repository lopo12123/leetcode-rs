struct Solution;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut stack = vec![];
        for char in preorder.split(",") {
            if char == "#" {
                stack.push(false);
            } else {
                stack.push(true);
            }

            loop {
                let len = stack.len();
                if len >= 3 && stack[len - 3] && !stack[len - 2] && !stack[len - 1] {
                    stack.pop();
                    stack.pop();
                    stack.pop();
                    stack.push(false);
                } else {
                    break;
                }
            }
        }

        stack.len() == 1 && stack[0] == false
    }
}

#[test]
fn t() {
    Solution::is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".to_string());
}