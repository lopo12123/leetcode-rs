struct Solution;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        // shadowing s1 and s2
        // since all the characters are lowercase English letters, this is enough (otherwise using 'Vec::from(s1.chars())')
        // let s1 = Vec::from(s1);
        // let s2 = Vec::from(s2);
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();

        // pointers for s1 and s2
        let mut ptr1 = 0;
        let mut ptr2 = 0;

        // counters for s1 and s2
        let mut count1 = 0;
        let mut count2 = 0;

        while count1 < n1 {
            if s1[ptr1] == s2[ptr2] {
                ptr2 += 1;
            }

            // move pointer
            ptr1 += 1;

            // reset if overflow
            if ptr1 == s1.len() {
                ptr1 = 0;
                count1 += 1;
            }
            if ptr2 == s2.len() {
                ptr2 = 0;
                count2 += 1;
            }

            // if the initial state is reached again,
            // calculations are performed without repeating the previous steps.
            if ptr1 == 0 && ptr2 == 0 {
                let repeatable = (n1 - count1) / count1;
                count1 += repeatable * count1;
                count2 += repeatable * count2;
            }
        }

        count2 / n2
    }

    // 会超时
    pub fn method2(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        // special case, direct match
        if s1 == s2 {
            return n1 / n2;
        }

        let len_r = s1.len();
        let len_p = s2.len();

        // not enough resource
        if len_r * (n1 as usize) < len_p * (n2 as usize) {
            return 0;
        }

        // the resource we have
        let total_r = len_r * n1 as usize;
        let chars_r = s1.as_bytes();

        // the product's segments to produce
        let chars_p = s2.as_bytes();

        let mut count = 0;
        let mut ptr_r = 0usize;
        let mut ptr_p = 0usize;

        while ptr_r < total_r {
            // whether the character is useful
            if chars_r[ptr_r % len_r] == chars_p[ptr_p] {
                ptr_p += 1;
            }

            // a new segment is produced
            if ptr_p == len_p {
                ptr_p = 0;
                count += 1;
            }

            // anyway, step forward
            ptr_r += 1;
        }

        count / n2
    }
}


#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("{} tobe 1", Solution::get_max_repetitions("ab".to_string(), 1, "ab".to_string(), 1));
        println!("{} tobe 2", Solution::get_max_repetitions("acb".to_string(), 4, "ab".to_string(), 2));
        println!("{} tobe 1", Solution::get_max_repetitions("acb".to_string(), 1, "acb".to_string(), 1));
        println!("{} tobe 200000", Solution::get_max_repetitions("aahumeaylnlfdxfircvscxggbwkfnqduxwfnfozvsrtkjprepggxrpnrvystmwcysyycqpevikeffmznimkkasvwsrenazkycxf".to_string(), 1000000, "aac".to_string(), 10));
    }

    #[test]
    fn elapse() {
        let now = std::time::Instant::now();

        // let r = Solution::get_max_repetitions("aahumeaylnlfdxfircvscxggbwkfnqduxwfnfozvsrtkjprepggxrpnrvystmwcysyycqpevikeffmznimkkasvwsrenazkycxf".to_string(), 1000000, "aac".to_string(), 10);
        let r = Solution::method2("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(), 1000000, "a".to_string(), 1000000);
        let elapsed = now.elapsed();

        println!("r: {r}, e: {:?}", elapsed);
    }

    #[test]
    fn tt() {
        let str1 = "aahumeaylnlfdxfircvscxggbwkfnqduxwfnfozvsrtkjprepggxrpnrvystmwcysyycqpevikeffmznimkkasvwsrenazkycxf".to_string();
        let str2 = "aahumeaylnlfdxfircvscxggbwkfnqduxwfnfozvsrtkjprepggxrpnrvystmwcysyycqpevikeffmznimkkasvwsrenazkycxf".to_string();
        let str3 = "aahumeaylnlfdxfircvscxggbwkfnqduxwfnfozvsrtkjprepggxrpnrvystmwcysyycqpevikeffmznimkkasvwsrenazkycxf".to_string();
        let str4 = "aahumeaylnlfdxfircvscxggbwkfnqduxwfnfozvsrtkjprepggxrpnrvystmwcysyycqpevikeffmznimkkasvwsrenazkycxf".to_string();

        let t1 = std::time::Instant::now();
        let _ = str1.as_bytes();
        let elapsed1 = t1.elapsed();

        let t2 = std::time::Instant::now();
        let _ = str2.chars();
        let elapsed2 = t2.elapsed();

        let t3 = std::time::Instant::now();
        let _ = str3.chars().collect::<Vec<char>>();
        let elapsed3 = t3.elapsed();

        let t4 = std::time::Instant::now();
        let _ = Vec::from(str4);
        let elapsed4 = t4.elapsed();

        println!("{:?} | {:?} | {:?} | {:?}", elapsed1, elapsed2, elapsed3, elapsed4);
    }
}