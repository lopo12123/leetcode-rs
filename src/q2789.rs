struct Solution;

impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        let (max, sum) = nums.iter().rfold((0i64, 0i64), |(mut max, mut sum), &curr| {
            if curr as i64 > sum {
                max = max.max(sum);
                sum = curr as i64;
            } else {
                sum += curr as i64;
            }

            (max, sum)
        });

        max.max(sum)
    }
}

#[test]
fn t() {
    let r = Solution::max_array_value(vec![2, 3, 7, 9, 3]);
    println!("{}", r);
}