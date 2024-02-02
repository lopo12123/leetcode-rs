struct Solution;

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        // 取走造成的差距
        let mut diff = alice_values.iter().zip(bob_values.iter()).map(|(&a, &b)| (a + b, a, b)).collect::<Vec<(i32, i32, i32)>>();
        diff.sort_unstable_by(|a, b| {
            a.0.cmp(&b.0)
        });

        println!("{:?}", diff);

        let mut alice_score = 0;
        let mut bob_score = 0;

        -1
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("1 = {}", Solution::stone_game_vi(vec![1, 3], vec![2, 1]));
    }
}