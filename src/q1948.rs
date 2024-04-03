use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hasher};

struct Solution;

#[derive(Debug)]
struct Entry {
    idx: usize,
    /// 子树的 hash 值 (不包含当前节点的信息)
    hash: u64,
    children: HashMap<String, Box<Entry>>,
}

const INVALID_INDEX: usize = 20001;

impl Solution {
    fn hash_entry(node: &mut Entry) -> u64 {
        if node.children.is_empty() {
            // 叶子节点, 直接使用0作为 hash
            0
        } else {
            let mut hasher = DefaultHasher::new();
            // 非叶子节点, 对每个子树的 (key, hash) 作为整体排序后写入 hasher
            let mut children_hash = vec![];
            for (key, child) in node.children.iter_mut() {
                children_hash.push((key, Solution::hash_entry(child)));
            }

            // 只需要保证相同的规则排序即可
            children_hash.sort();

            for (child_key, child_hash) in children_hash {
                hasher.write(child_key.as_bytes());
                hasher.write_u64(child_hash);
            }

            node.hash = hasher.finish();
            node.hash
        }
    }

    fn count_entry(node: &Entry, freq: &mut HashMap<u64, u32>) {
        *freq.entry(node.hash).or_insert(0) += 1;
        for child in node.children.values() {
            Solution::count_entry(child, freq);
        }
    }

    /// 根据子树是否 "包含 非空且相同的 子文件夹 集合 并具有相同的子文件夹结构", 进行标记
    /// 默认先将全部标记为删除, 再递归判断是否需要保留
    fn check_duplicate(
        node: &Entry,
        state: &mut Vec<bool>,
        freq: &HashMap<u64, u32>,
        // 叶子节点跟随父节点状态
        keep_parent: bool,
    ) {
        // 叶子节点直接返回
        if node.children.is_empty() {
            // 叶子节点一定是有效的 idx
            state[node.idx] = keep_parent;
            return;
        }
        // if node.children.is_empty() { return; }

        // 如果hash频次等于1, 则说明子结构是唯一的, 标记为保留
        if node.idx == INVALID_INDEX || freq.get(&node.hash).unwrap() == &1 {
            // 自身标记为保留
            if node.idx != INVALID_INDEX {
                state[node.idx] = true;
            }

            // 递归判断子节点
            for child in node.children.values() {
                Solution::check_duplicate(child, state, freq, true);
            }
        }
        // 如果某个节点被标记为删除, 则其子树全部删除, 无需再递归判断
    }

    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // 重建树
        let mut root = Entry {
            idx: 0,
            hash: 0,
            children: HashMap::new(),
        };
        for (path_idx, segs) in paths.iter().enumerate() {
            let mut cur = &mut root;
            let seg_leaf_idx = segs.len() - 1;
            for (seg_idx, seg) in segs.into_iter().enumerate() {
                cur = cur.children
                    .entry(seg.clone())
                    .or_insert(Box::new(Entry {
                        idx: INVALID_INDEX,
                        hash: 0,
                        children: HashMap::new(),
                    }));

                // 叶子节点记忆idx信息
                if seg_idx == seg_leaf_idx {
                    cur.idx = path_idx;
                }
            }
        }

        // 计算 hash
        Solution::hash_entry(&mut root);

        // 统计频率 {hash => count}
        let mut freq = HashMap::new();
        Solution::count_entry(&root, &mut freq);

        // 统计保留项的下标
        // 根节点是虚拟节点, 不参与判断 (避免其 idx=0 被误删或保留)
        let mut state = vec![false; paths.len()];
        for tree in root.children.values() {
            Solution::check_duplicate(tree, &mut state, &freq, true);
        };

        // println!("{:#?}", root);
        // println!("{:#?}", freq);
        // println!("{:#?}", state);

        // 生成结果
        let mut result = vec![];
        for (idx, segs) in paths.into_iter().enumerate() {
            if state[idx] {
                result.push(segs);
            }
        }
        result
    }
}

#[test]
fn t() {
    // let r = Solution::delete_duplicate_folder(vec![
    //     vec!["a".to_string()],
    //     vec!["c".to_string()],
    //     vec!["d".to_string()],
    //     vec!["a".to_string(), "b".to_string()],
    //     vec!["c".to_string(), "b".to_string()],
    //     vec!["d".to_string(), "a".to_string()],
    // ]);
    // println!("{:?}", r);

    let r = Solution::delete_duplicate_folder(vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["c".to_string(), "d".to_string()],
        vec!["c".to_string()],
        vec!["a".to_string()],
    ]);
    println!("{:?}", r);
}