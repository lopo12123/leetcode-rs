struct Solution;

impl Solution {
    fn pop_until_then_push(arr: &mut Vec<i32>, threshold: i32) -> i32 {
        let mut count = 0;

        while arr.len() > 0 && arr.last().unwrap() < &threshold {
            arr.pop();
            count += 1;
        }

        if arr.len() > 0 { count += 1; }
        arr.push(threshold);

        count
    }

    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];

        for i in (0..heights.len()).rev() {
            ans.push(Solution::pop_until_then_push(&mut stack, heights[i]));
        }

        ans.reverse();
        ans
    }
}


#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("{:?}", Solution::can_see_persons_count(vec![10, 6, 8, 5, 11, 9]));
        println!("{:?}", Solution::can_see_persons_count(vec![5, 1, 2, 3, 10]));
        println!("{:?}", Solution::can_see_persons_count(vec![6, 8, 3, 1, 7, 5, 9, 2, 4]));
    }
}