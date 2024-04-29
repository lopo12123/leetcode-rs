struct Solution;

impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row = mat.len();
        let col = mat[0].len();

        let mut lines = vec![vec![]; col + row - 1];
        for r in 0..row {
            for c in 0..col {
                lines[r - c + col - 1].push(mat[r][c]);
            }
        }

        for line in &mut lines {
            line.sort_unstable_by(|a, b| b.cmp(a));
        }

        println!("{:?}", lines);

        let mut res = vec![vec![0; col]; row];
        for r in 0..row {
            for c in 0..col {
                res[r][c] = lines[r - c + col - 1].pop().unwrap();
            }
        }
        res
    }
}