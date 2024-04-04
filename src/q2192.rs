use std::collections::{HashSet};

struct Solution;

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 每个节点的直接父节点
        let mut parents = vec![vec![]; n as usize];
        // 每个节点的直接子节点
        let mut children = vec![vec![]; n as usize];
        for edge in edges {
            parents[edge[1] as usize].push(edge[0] as usize);
            children[edge[0] as usize].push(edge[1] as usize);
        }

        // 当前层级需要遍历的节点
        let mut batch = HashSet::new();
        for i in 0..n as usize {
            if parents[i].is_empty() {
                batch.insert(i);
            }
        }

        // 每个节点的所有祖先节点
        let mut re_set = vec![HashSet::new(); n as usize];

        loop {
            // 没有节点需要遍历, 退出
            if batch.is_empty() { break; }

            let mut next_batch = HashSet::new();

            // 遍历当前层级的所有节点
            for i in batch {
                // 用于收集当前节点的所有祖先节点
                let mut ancestors = HashSet::new();
                for &parent in &parents[i] {
                    // 添加直接父节点
                    ancestors.insert(parent as i32);
                    // 添加该父节点的祖先节点
                    ancestors.extend(re_set[parent].iter());
                }
                // 赋值
                re_set[i] = ancestors;

                // 添加当前节点的所有子节点
                next_batch.extend(&children[i]);
            }

            // 更新层级
            batch = next_batch;
        }

        re_set.into_iter().map(|set| {
            let mut acc = set.into_iter().collect::<Vec<i32>>();
            acc.sort_unstable();
            acc
        }).collect()
    }
}