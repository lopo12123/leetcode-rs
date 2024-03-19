struct Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        // 最终取到的最小数字
        let mut min = nums[k];
        // (左界, 左侧到当前位置的最小值)
        let mut left = (k, min);
        let mut right = (k, min);

        for p1 in (0..k).rev() {
            if nums[p1] >= min {
                // 当前数字大于等于最小数字, 直接包括在内
                left.0 = p1;
            } else {
                // 始终更新左侧最小值
                left.1 = left.1.min(nums[p1]);

                // 进行判断
                if (k - p1 + 1) as i32 * left.1 > (k - left.0 + 1) as i32 * min {
                    min = left.1;
                    left.0 = p1;
                }
            }
        }

        for p2 in (k + 1)..nums.len() {
            if nums[p2] >= min {
                // 当前数字大于等于最小数字, 直接包括在内
                right.0 = p2;
            } else {
                // 始终更新右侧最小值
                right.1 = right.1.min(nums[p2]);

                // 进行判断
                if (p2 - k + 1) as i32 * right.1 > (right.0 - k + 1) as i32 * min {
                    min = right.1;
                    right.0 = p2;
                }
            }
        }

        (right.0 - left.0 + 1) as i32 * min
    }
}

#[test]
fn t() {
    // let r = Solution::maximum_score(vec![1, 4, 3, 7, 4, 5], 3);
    // println!("r tobe 15, is {}", r);

    // let r = Solution::maximum_score(vec![5, 5, 4, 5, 4, 1, 1, 1], 0);
    // println!("r tobe 20, is {}", r);

    let r = Solution::maximum_score(vec![6569, 9667, 3148, 7698, 1622, 2194, 793, 9041, 1670, 1872], 5);
    println!("r tobe 9732, is {}", r);
}