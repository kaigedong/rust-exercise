use std::cell::RefCell;
use std::rc::Rc;

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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut out = vec![];
        let root = root.as_deref();
        Self::postorder_traversal_helper(root, &mut out);
        out
    }
    fn postorder_traversal_helper(root: Option<&RefCell<TreeNode>>, out: &mut Vec<i32>) {
        if let Some(root) = root {
            Self::postorder_traversal_helper(root.borrow().left.as_deref(), out);
            Self::postorder_traversal_helper(root.borrow().right.as_deref(), out);
            out.push(root.borrow().val);
        }
    }
}
