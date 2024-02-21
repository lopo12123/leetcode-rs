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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = inorder.len();

        if len == 0 { return None; }

        let root_val = postorder[len - 1];
        let root_idx = inorder.iter().position(|&v| v == root_val).unwrap();

        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left: Solution::build_tree(inorder[0..root_idx].to_vec(), postorder[0..root_idx].to_vec()),
            right: Solution::build_tree(inorder[root_idx + 1..].to_vec(), postorder[root_idx..len - 1].to_vec()),
        })))
    }
}