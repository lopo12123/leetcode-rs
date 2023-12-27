// see https://leetcode.cn/problems/determine-the-winner-of-a-bowling-game/description/

struct Solution;

impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        fn score(hits: Vec<i32>) -> i32 {
            // 每次得到 10 分即可双倍两回合
            let mut bonus_round = 0;

            // 总分
            let mut score = 0;

            for hit in hits {
                score += hit * if bonus_round > 0 { 2 } else { 1 };
                // score += hit + if bonus_round > 0 { hit } else { 0 };
                bonus_round -= 1;

                // 刷新但不叠加
                if hit == 10 { bonus_round = 2 }
            }

            score
        }

        let score1 = score(player1);
        let score2 = score(player2);
        if score1 < score2 { 2 } else if score1 > score2 { 1 } else { 0 }

        // match score(player1).cmp(&score(player2)) {
        //     std::cmp::Ordering::Less => 2,
        //     std::cmp::Ordering::Greater => 1,
        //     std::cmp::Ordering::Equal => 0,
        // }
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn run() {
        let test_cases = vec![
            (vec![4, 10, 7, 9], vec![6, 5, 2, 3], 1),
            (vec![3, 5, 7, 6], vec![8, 10, 10, 2], 2),
            (vec![2, 3], vec![4, 1], 0),
            (vec![5, 6, 1, 10], vec![5, 1, 10, 5], 2),
        ];

        for test_case in test_cases {
            let player1 = test_case.0;
            let player2 = test_case.1;

            assert_eq!(Solution::is_winner(player1, player2), test_case.2);
        }

        println!("PASS");
    }
}