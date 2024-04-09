use std::collections::{HashMap, HashSet};

struct ThroneInheritance {
    /// 国王名字
    king: String,
    /// 死掉的人的集合
    dead: HashSet<String>,
    /// 树
    tree: HashMap<String, Vec<String>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ThroneInheritance {
    fn new(kingName: String) -> Self {
        ThroneInheritance {
            king: kingName.clone(),
            dead: HashSet::new(),
            tree: HashMap::from([(kingName, vec![])]),
        }
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        // 没有创建过 children 的人不会出现在 key 中, 需要使用 entry
        self.tree.entry(parent_name).or_insert(vec![]).push(child_name);
    }

    fn death(&mut self, name: String) {
        self.dead.insert(name);
    }

    fn successor(&self, x: &String, cur_order: &mut Vec<String>) {
        // 没死就进入顺位
        if !self.dead.contains(x) { cur_order.push(x.to_owned()); }

        if let Some(children) = self.tree.get(x) {
            for child in children {
                self.successor(child, cur_order);
            }
        }
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        let mut final_order = vec![];
        self.successor(&self.king, &mut final_order);
        final_order
    }
}