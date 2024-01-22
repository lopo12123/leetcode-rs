struct Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut n = num;
        let mut nums = vec![];
        while n > 0 {
            nums.push(n % 10);
            n /= 10;
        }

        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        // 找到需要交换的一个数字的位置
        let mut swap_source_idx = nums.len();
        for i in (0..nums.len()).rev() {
            if nums[i] != sorted_nums[i] {
                swap_source_idx = i;
                break;
            }
        }

        // 无需交换, 直接返回原数据
        if swap_source_idx == nums.len() {
            return num;
        }

        // 找到交换的目标
        for i_back in 0..nums.len() {
            if nums[i_back] == sorted_nums[swap_source_idx] {
                let source = nums[swap_source_idx];
                nums[swap_source_idx] = nums[i_back];
                nums[i_back] = source;
            }
        }

        nums.iter().rfold(0, |mut v, curr| 10 * v + *curr)
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("7236 = {}", Solution::maximum_swap(2736));
        println!("9973 = {}", Solution::maximum_swap(9973));
    }
}