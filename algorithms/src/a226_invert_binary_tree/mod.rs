use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root_ref = root.as_deref();
        Self::invert_tree_helper(root_ref);
        root
    }
    fn invert_tree_helper(root: Option<&RefCell<TreeNode>>) {
        if let Some(root) = root {
            let root = &mut *root.borrow_mut();
            mem::swap(&mut root.left, &mut root.right);
            Self::invert_tree_helper(root.left.as_deref());
            Self::invert_tree_helper(root.right.as_deref());
        }
    }
}
