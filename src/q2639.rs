struct Solution;

impl Solution {
    #[inline]
    fn width(val: i32) -> i32 {
        val.to_string().len() as i32
    }

    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let row = grid.len();
        let col = grid[0].len();

        let mut result = vec![0; col];
        for i in 0..row {
            for j in 0..col {
                result[j] = result[j].max(Self::width(grid[i][j]));
            }
        }

        result
    }
}