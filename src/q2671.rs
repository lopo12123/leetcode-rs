use std::collections::{HashMap, HashSet};

struct FrequencyTracker {
    n2f: HashMap<i32, i32>,
    f2n: HashMap<i32, HashSet<i32>>,
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
        // 频率加一, 如果没有则默认为1
        let f_now = self.n2f.entry(number).and_modify(|f| *f += 1).or_insert(1);
        // 在新频率中加入该数字
        self.f2n.entry(*f_now).or_insert(HashSet::new()).insert(number);
        // 如果频率不为1, 说明该数字之前已经存在, 需要删除旧的频率中的该数字
        if *f_now > 1 {
            self.f2n.get_mut(&(*f_now - 1)).map(|set| set.remove(&number));
        }
    }

    fn delete_one(&mut self, number: i32) {
        // 如果有该数字则执行操作
        if let Some(spec) = self.n2f.get_mut(&number) {
            // 如果频率为0, 说明该数字已经被删除, 直接返回
            if *spec == 0 {
                return;
            }

            // 原有频率中删去该数字
            self.f2n.get_mut(spec).map(|set| set.remove(&number));
            // 频率减一
            *spec -= 1;
            // 新频率中加入该数字
            self.f2n.get_mut(spec).map(|set| set.insert(number));
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.f2n.get(&frequency).map_or(false, |set| !set.is_empty())
    }
}