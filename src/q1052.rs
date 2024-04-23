struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let minutes = minutes as usize;

        if minutes >= customers.len() {
            // 全部满意
            customers.iter().sum()
        } else {
            let len = customers.len();

            let mut rev = 0;
            for i in 0..minutes {
                if grumpy[i] == 1 {
                    rev += customers[i];
                }
            }

            let mut rev_max = rev;
            for prev_start in 0..len - minutes {
                if grumpy[prev_start] == 1 {
                    rev -= customers[prev_start];
                }
                if grumpy[prev_start + minutes] == 1 {
                    rev += customers[prev_start + minutes];
                }
                rev_max = rev_max.max(rev);
            }

            let mut sat = 0;
            for i in 0..len {
                if grumpy[i] == 0 {
                    sat += customers[i];
                }
            }
            sat + rev_max
        }
    }
}