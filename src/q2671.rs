use std::collections::{HashMap, HashSet};

struct FrequencyTracker {
    n2f: HashMap<i32, i32>,
    f2n: HashMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrequencyTracker {
    fn new() -> Self {
        FrequencyTracker { n2f: HashMap::new(), f2n: HashMap::new() }
    }

    fn add(&mut self, number: i32) {
        // 修改该数字的频率
        let f_new = self.n2f.entry(number).and_modify(|v| *v += 1).or_insert(1);
        if *f_new > 1 {
            // 旧频率对应数字-1
            self.f2n.get_mut(&(*f_new - 1)).map(|v| *v -= 1);
        }
        // 新频率对应数字+1
        self.f2n.entry(*f_new).and_modify(|v| *v += 1).or_insert(1);
    }

    fn delete_one(&mut self, number: i32) {
        // 存在且频率大于等于1才执行操作
        if self.n2f.get(&number).is_some_and(|v| *v >= 1) {
            // 修改该数字的频率
            let freq = self.n2f.get_mut(&number).unwrap();
            // 旧频率对应数字-1
            self.f2n.get_mut(freq).map(|v| *v -= 1);
            *freq -= 1;
            // 新频率对应数字+1
            self.f2n.get_mut(freq).map(|v| *v += 1);
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.f2n.get(&frequency).map_or(false, |v| *v > 0)
    }

    pub fn log(&self) {
        println!("n2f: {:?}\nf2n: {:?}\n======", self.n2f, self.f2n);
    }
}