struct Solution;

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut right = HashMap::new();
        for num in &nums {
            right.entry(num).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut result = vec![];
        let mut left = HashMap::new();
        for num in &nums {
            left.entry(num).and_modify(|v| *v += 1).or_insert(1);

            match right.get(num) {
                None => {}
                Some(v) => match v {
                    1 => { right.remove(num); }
                    _ => { right.insert(num, *v - 1); }
                },
            };
            result.push(left.len() as i32 - right.len() as i32)
        }

        result
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("In:  {:?}\nOut: {:?}", vec![-3, -1, 1, 3, 5], Solution::distinct_difference_array(vec![1, 2, 3, 4, 5]));
        println!("In:  {:?}\nOut: {:?}", vec![-2, -1, 0, 2, 3], Solution::distinct_difference_array(vec![3, 2, 3, 4, 2]));
    }
}