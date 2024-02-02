struct Solution;

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut diff = alice_values.iter().zip(bob_values.iter()).map(|(&a, &b)| (a + b, a, b)).collect::<Vec<(i32, i32, i32)>>();
        diff.sort_unstable_by(|a, b| {
            b.0.cmp(&a.0)
        });

        let mut baseline = 0;
        for (idx, &(_, a, b)) in diff.iter().enumerate() {
            if idx % 2 == 0 { baseline += a } else { baseline -= b }
        }

        baseline.signum()
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