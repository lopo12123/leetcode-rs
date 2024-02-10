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

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut re = vec![];

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(inner) = node {
                let inner = inner.borrow();
                dfs(&inner.left, result);
                result.push(inner.val);
                dfs(&inner.right, result);
            }
        }
        dfs(&root, &mut re);

        re
    }
}

struct Solution;

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        todo!()
    }
}