struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut p_new = 0;

        for ptr in 1..nums.len() {
            if nums[ptr] != nums[p_new] {
                p_new += 1;
                nums[p_new] = nums[ptr];
            }
        }

        p_new as i32 + 1
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        let mut nums = vec![1, 1, 2];
        println!("2 = {} | {:?}", Solution::remove_duplicates(&mut nums), &nums);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        println!("5 = {} | {:?}", Solution::remove_duplicates(&mut nums), &nums);
    }
}