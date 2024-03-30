struct Solution;

impl Solution {
    pub fn minimum_added_coins(mut coins: Vec<i32>, target: i32) -> i32 {
        coins.sort_unstable();

        let mut count = 0;
        let mut reachable = 0;

        let mut ptr = 0;
        loop {
            if reachable >= target {
                break;
            }

            let coin = *coins.get(ptr).unwrap_or(&0);

            if coin == 0 || coin > reachable + 1 {
                count += 1;
                reachable += reachable + 1;
            } else {
                reachable += coin;

                // 只有消耗掉当前coin才能继续往后走
                ptr += 1;
            }
        }

        count
    }
}