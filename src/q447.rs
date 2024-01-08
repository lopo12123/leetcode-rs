struct Solution;

impl Solution {
    /// cause trait `Eq` is not implemented for `f32`, we use pow2 as hashKey
    fn distance_pow(p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
        (p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2)
    }

    /// pick 2 from n
    fn pick_n2(n: i32) -> i32 {
        if n < 2 { 0 } else { n * (n - 1) / 2 }
    }

    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for i in 0..points.len() {
            let fixed = &points[i];

            let mut map = std::collections::HashMap::new();
            for j in 0..points.len() {
                // skip self
                if i == j { continue; }

                let d = Solution::distance_pow(fixed, &points[j]);
                map.insert(d, map.get(&d).unwrap_or(&0) + 1);
            }

            for x in map.values() {
                result += Solution::pick_n2(*x);
            }
        }

        result * 2
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("{:?}", Solution::number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]]));
    }
}