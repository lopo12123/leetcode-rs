struct Solution;

impl Solution {
    pub fn distinct_integers(n: i32) -> i32 {
        (n - 1).max(1)
    }
}