struct Solution;

impl Solution {
    fn score(hits: Vec<i32>) -> i32 {
        let mut score = 0;

        let mut pre1 = 0;
        let mut pre2 = 0;
        for hit in hits {
            let rate = if pre1 == 10 || pre2 == 10 { 2 } else { 1 };

            score += hit * rate;

            pre2 = pre1;
            pre1 = hit;
        }

        score
    }

    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        match (Solution::score(player1) - Solution::score(player2)).signum() {
            1 => 1,
            -1 => 2,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn run() {
        let test_cases = vec![
            (vec![4, 10, 7, 9], vec![6, 5, 2, 3]),
            (vec![3, 5, 7, 6], vec![8, 10, 10, 2]),
            (vec![2, 3], vec![4, 1]),
            (vec![5, 6, 1, 10], vec![5, 1, 10, 5]),
        ];

        for test_case in test_cases {
            let player1 = test_case.0;
            let player2 = test_case.1;

            println!("result: {}", Solution::is_winner(player1, player2));
        }
    }
}