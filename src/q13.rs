struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;

        // - 0: nothing
        // - 1: I
        // - 2: X
        // - 3: C
        let mut flag = 0;

        for char in s.chars() {
            match char {
                'I' => {
                    result += 1;
                    flag = 1;
                }
                'V' => {
                    if flag == 1 {
                        // 1 is pre-added in the previous loop (so here add 4-1=3)
                        result += 3;
                    } else {
                        result += 5;
                    }
                    flag = 0;
                }
                'X' => {
                    if flag == 1 {
                        result += 8;
                        flag = 0;
                    } else {
                        result += 10;
                        flag = 2;
                    }
                }
                'L' => {
                    if flag == 2 {
                        result += 30;
                    } else {
                        result += 50;
                    }
                    flag = 0;
                }
                'C' => {
                    if flag == 2 {
                        result += 80;
                        flag = 0;
                    } else {
                        result += 100;
                        flag = 3;
                    }
                }
                'D' => {
                    if flag == 3 {
                        result += 300;
                    } else {
                        result += 500;
                    }
                    flag = 0;
                }
                'M' => {
                    if flag == 3 {
                        result += 800;
                    } else {
                        result += 1000;
                    }
                    flag = 0;
                }
                _ => unreachable!()
            }
        }

        result
    }
}


#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        println!("3 = {}", Solution::roman_to_int("III".to_string()));
        println!("4 = {}", Solution::roman_to_int("IV".to_string()));
        println!("9 = {}", Solution::roman_to_int("IX".to_string()));
        println!("58 = {}", Solution::roman_to_int("LVIII".to_string()));
        println!("1994 = {}", Solution::roman_to_int("MCMXCIV".to_string()));
    }
}