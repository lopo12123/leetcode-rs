struct Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        // boundary
        let mut boundary = (k, k);
        // min in [bl, br]
        let mut min = nums[k];

        for pl in (0..k).rev() {
            if nums[pl] >= min {
                boundary.0 = pl;
            } else {
                break;
            }
        }
        for pr in (k + 1)..nums.len() {
            if nums[pr] >= min {
                boundary.1 = pr;
            } else {
                break;
            }
        }

        // pointer -- candidates
        let mut pl = boundary.0;
        let mut pr = boundary.1;
        let mut score = (boundary.1 - boundary.0 + 1) as i32 * min;
        loop {
            if pl == 0 && pr == nums.len() - 1 { break; }

            let vl = if pl == 0 { -1 } else { nums[pl - 1] };
            let vr = if pr == nums.len() - 1 { -1 } else { nums[pr + 1] };

            if vl > vr {
                min = min.min(vl);
                let new_score = (pr - pl + 2) as i32 * min;
                // println!("new_score={}, score={}, min={min}, ({pl}, {pr})", new_score, score);
                if new_score > score {
                    score = new_score;
                }

                pl -= 1;
            } else {
                min = min.min(vr);
                let new_score = (pr - pl + 2) as i32 * min;
                // println!("new_score={}, score={}, min={min}, ({pl}, {pr})", new_score, score);
                if new_score > score {
                    score = new_score;
                }

                pr += 1;
            }
        }

        score
    }
}

#[test]
fn t() {
    let r = Solution::maximum_score(vec![1, 4, 3, 7, 4, 5], 3);
    println!("r tobe 15, is {}", r);

    let r = Solution::maximum_score(vec![5, 5, 4, 5, 4, 1, 1, 1], 0);
    println!("r tobe 20, is {}", r);

    let r = Solution::maximum_score(vec![6569, 9667, 3148, 7698, 1622, 2194, 793, 9041, 1670, 1872], 5);
    println!("r tobe 9732, is {}", r);

    let r = Solution::maximum_score(vec![5534, 378, 6709, 3383, 2247, 1491, 5119, 4055, 8735, 7227, 3322, 3788], 6);
    println!("r tobe 19932, is {}", r);
}