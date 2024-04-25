struct Solution;

impl Solution {
    pub fn distance_traveled(mut main_tank: i32, mut additional_tank: i32) -> i32 {
        let mut total = 0;
        while main_tank >= 5 {
            main_tank -= 5;
            total += 5;
            if additional_tank > 0 {
                main_tank += 1;
                additional_tank -= 1;
            }
        }

        10 * (total + main_tank)
    }
}