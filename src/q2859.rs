use std::ops::BitAndAssign;

struct Solution;

impl Solution {
    fn calc_bits(mut num: usize) -> i32 {
        let mut n = 0;
        while num > 0 {
            n += num % 2;
            num /= 2;
        }
        n as i32
    }

    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        for (idx, num) in nums.iter().enumerate() {
            if Solution::calc_bits(idx) == k {
                sum += num;
            }
        }
        sum
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("13 = {}", Solution::sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1));
        println!("1 = {}", Solution::sum_indices_with_k_set_bits(vec![4, 3, 2, 1], 2));
    }
}