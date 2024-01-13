#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    const PATCH: [i32; 3] = [2, 0, 1];

    pub fn add_minimum(word: String) -> i32 {
        let mut count = 0;
        let mut prev = 'c' as u32;

        for ch in word.chars() {
            let curr = ch as u32;

            count += Solution::PATCH[((3 + curr - prev) % 3) as usize];
            prev = curr;
        }

        match word.chars().last().unwrap() {
            'a' => count += 2,
            'b' => count += 1,
            _ => {}
        }

        count
    }
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn t() {
        println!("{}", 'a' as u32);
    }
}