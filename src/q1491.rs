struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut max = std::i32::MIN;
        let mut min = std::i32::MAX;
        let mut sum = 0;

        salary.iter().for_each(|&s| {
            max = max.max(s);
            min = min.min(s);
            sum += s;
        });

        (sum - max - min) as f64 / (salary.len() as f64 - 2.0)
    }
}