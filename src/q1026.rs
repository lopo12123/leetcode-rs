// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let max = Rc::new(RefCell::new(0));

        fn dfs(node: &Rc<RefCell<TreeNode>>, max_diff: Rc<RefCell<i32>>) -> (i32, i32) {
            let node = node.borrow();

            // 当前节点为根的树中的最小值和最大值
            let mut min = node.val;
            let mut max = node.val;

            if let Some(left) = &node.left {
                let (min_l, max_l) = dfs(left, max_diff.clone());
                min = min.min(min_l);
                max = max.max(max_l);

                // 计算左右diff
                let diff_min = (node.val - min_l).abs();
                let diff_max = (node.val - max_l).abs();
                // 更新最大的diff
                let mut max_diff_mut = max_diff.borrow_mut();
                *max_diff_mut = max_diff_mut.max(diff_min.max(diff_max));
            }

            if let Some(right) = &node.right {
                let (min_l, max_l) = dfs(right, max_diff.clone());
                min = min.min(min_l);
                max = max.max(max_l);

                // 计算当前节点对左右子树的最大diff
                let diff_min = (node.val - min_l).abs();
                let diff_max = (node.val - max_l).abs();

                // 更新最大的diff
                let mut max_diff_mut = max_diff.borrow_mut();
                *max_diff_mut = max_diff_mut.max(diff_min.max(diff_max));
            }

            (min, max)
        }
        dfs(&root.unwrap(), max.clone());

        let v = *max.borrow();
        v
    }
}

#[test]
fn t() {
    let v = Rc::new(RefCell::new(1));

    let mut v_mut = v.borrow_mut();
    *v_mut = 3;

    println!("{}", v_mut);
}