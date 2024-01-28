struct Solution;

impl Solution {
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a } else { gcd(b, a % b) }
        }

        (jug1_capacity + jug2_capacity >= target_capacity) && target_capacity % gcd(jug1_capacity, jug2_capacity) == 0
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        todo!()
    }
}