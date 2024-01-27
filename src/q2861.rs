struct Solution;

impl Solution {
    #[inline]
    fn total_cost(need: i32, plan: &Vec<i32>, stock: &Vec<i32>, cost: &Vec<i32>) -> i64 {
        plan.iter().enumerate().fold(0, |total, (idx, &this)| {
            total + (this as i64 * need as i64 - stock[idx] as i64).max(0) * cost[idx] as i64
        })
    }

    #[inline]
    fn check(budget: i32, need: i32, composition: &Vec<Vec<i32>>, stock: &Vec<i32>, cost: &Vec<i32>) -> bool {
        for plan in composition {
            if Solution::total_cost(need, plan, stock, cost) <= budget as i64 {
                return true;
            }
        }

        false
    }

    pub fn max_number_of_alloys(n: i32, k: i32, budget: i32, composition: Vec<Vec<i32>>, stock: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut min = 0;
        let mut max = 2 * 100_000_000;

        let mut r = 0;

        while min <= max {
            let attempt = (min + max) / 2;
            if Solution::check(budget, attempt, &composition, &stock, &cost) {
                min = attempt + 1;
                r = attempt;
            } else {
                max = attempt - 1;
            }
        }

        r
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("2 = {}", Solution::max_number_of_alloys(3, 2, 15, vec![vec![1, 1, 1], vec![1, 1, 10]], vec![0, 0, 0], vec![1, 2, 3]));
        // println!("5 = {}", Solution::max_number_of_alloys(3, 2, 15, vec![vec![1, 1, 1], vec![1, 1, 10]], vec![0, 0, 100], vec![1, 2, 3]));
    }
}