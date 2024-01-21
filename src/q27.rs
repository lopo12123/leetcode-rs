struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32

        // let mut ptr = 0;
        //
        // for x in 0..nums.len() {
        //     if nums[x] != val {
        //         nums[ptr] = nums[x];
        //         ptr += 1;
        //     }
        // }
        //
        // ptr as i32
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        println!("{} | {:?}", Solution::remove_element(&mut nums, 2), &nums);
    }
}