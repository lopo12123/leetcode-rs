struct Solution;

impl Solution {
    fn bit_count(mut n: i32) -> Vec<i32> {
        let mut res = vec![];
        while n > 0 {
            res.push(n % 2);
            n /= 2;
        }
        res
    }

    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut bit_sum = vec![0; 32];

        let mut bits = 0;
        for n in nums {
            let state = Solution::bit_count(n);
            if state.len() > bits {
                bits = state.len();
            }
            for i in 0..state.len() {
                bit_sum[i] += state.get(i).unwrap_or(&0);
            }
        }

        let mut kor = 0;
        for i in 0..bits {
            kor += if bit_sum[i] >= k { 1 << i } else { 0 };
        }

        kor
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        // 1110
        // 0111
        // 1100
        // 1001
        // 1000
        // 1001
        // 0001
        // 1111
        //
        // => 1101

        let r = Solution::find_k_or(vec![14, 7, 12, 9, 8, 9, 1, 15], 4);
        println!("r: {}", r);
    }
}