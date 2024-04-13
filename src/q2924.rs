struct Solution;

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut fails = vec![0; n as usize];

        for round in edges {
            let fail = round[1] as usize;
            fails[fail] += 1;
        }

        let mut winner_idx = -1;
        for (idx, &v) in fails.iter().enumerate() {
            if v == 0 {
                if winner_idx == -1 {
                    winner_idx = idx as i32;
                } else {
                    return -1;
                }
            }
        }

        winner_idx
    }
}