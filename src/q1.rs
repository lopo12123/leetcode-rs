struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // { value => index }
        let mut scanned: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

        for enu in nums.iter().enumerate() {
            let need = target - enu.1;
            if let Some(entry) = scanned.get_key_value(&need) {
                return vec![*entry.1, enu.0 as i32];
            }

            scanned.insert(*enu.1, enu.0 as i32);
        }

        // use macro rather `vec![]`
        unreachable!()
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
    }
}