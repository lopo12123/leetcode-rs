struct Solution;

impl Solution {
    pub fn min_operations_max_profit(customers: Vec<i32>, boarding_cost: i32, running_cost: i32) -> i32 {
        if running_cost >= boarding_cost * 4 {
            -1
        } else {
            // The money earned till now
            let mut earned = 0;

            // A snapshot of the maximum case (max_earned, max_round)
            let mut max_shot = (0, 0);

            // Number of customers waiting
            let mut waiting = customers[0];

            let mut round = 0;
            loop {
                // Next round
                round += 1;

                // The number of customers that can be served in this round
                let wipe = if waiting < 4 { waiting } else { 4 };

                // Processing ...
                waiting -= wipe;
                earned += wipe * boarding_cost - running_cost;

                // Compare our earnings
                if earned > max_shot.0 {
                    max_shot = (earned, round);
                }

                // Welcome the next customer
                if round < customers.len() {
                    waiting += customers[round];
                }

                // No more customers and no more waiting, we are done
                if round >= customers.len() && waiting == 0 {
                    break;
                }
            }

            if max_shot.0 > 0 {
                max_shot.1 as i32
            } else {
                -1
            }
        }
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("{} tobe 3", Solution::min_operations_max_profit(vec![8, 3], 5, 6));
        println!("{} tobe 7", Solution::min_operations_max_profit(vec![10, 9, 6], 6, 4));
        println!("{} tobe -1", Solution::min_operations_max_profit(vec![3, 4, 0, 5, 1], 1, 92));
    }
}