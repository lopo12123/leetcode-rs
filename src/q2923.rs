struct Solution;

impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let max = (grid.len() - 1) as i32;

        for (num, row) in grid.into_iter().enumerate() {
            let sum = row.iter().sum::<i32>();
            if sum == max {
                return num as i32;
            }
        }

        -1
    }
}