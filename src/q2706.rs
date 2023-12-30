struct Solution;

#[allow(unused)]
impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut p1 = i32::MAX;
        let mut p2 = i32::MAX;

        for p in prices {
            if p < p1 {
                p2 = p1;
                p1 = p;
            } else if p < p2 {
                p2 = p;
            }
        }

        if p1 == i32::MAX || p2 == i32::MAX {
            money
        } else {
            let remain = money - p1 - p2;
            if remain < 0 {
                money
            } else {
                remain
            }
        }
    }
}