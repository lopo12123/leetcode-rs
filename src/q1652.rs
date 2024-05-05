struct Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let len = code.len();

        let mut re = vec![0; len];
        if k == 0 {
            return re;
        }

        let code = if k > 0 {
            [&code[1..], &code[0..(k as usize + 1)]]
        } else {
            [&code[len - (-k as usize)..], &code]
        }.concat();
        let k = if k > 0 { k as usize } else { -k as usize };

        for i in 0..len {
            re[i] = code[i..i + k].iter().sum();
        }

        re
    }
}