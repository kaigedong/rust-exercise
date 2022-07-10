use std::cell::RefCell;
use std::rc::Rc;

mod recursive_2;

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

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut out = vec![];
        Self::is_valid_bst_helper(root.as_deref(), &mut out);
        if out.len() <= 1 {
            return true;
        }
        for i in 1..out.len() {
            if out[i] <= out[i - 1] {
                return false;
            }
        }
        true
    }
    fn is_valid_bst_helper(root: Option<&RefCell<TreeNode>>, out: &mut Vec<i32>) {
        if let Some(root) = root {
            let root = root.borrow();
            let left = root.left.as_deref();
            let right = root.right.as_deref();

            Self::is_valid_bst_helper(left, out);
            out.push(root.val);
            Self::is_valid_bst_helper(right, out);
        }
    }
}
