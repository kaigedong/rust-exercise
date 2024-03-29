use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn kth_largest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut out = vec![];
        Self::kth_largest_helper(root.as_deref(), &mut out);
        out[(k - 1) as usize]
    }

    // right, node, left, return the position of node.
    fn kth_largest_helper(root: Option<&RefCell<TreeNode>>, out: &mut Vec<i32>) {
        if let Some(root) = root {
            let root = root.borrow();
            let left = root.left.as_deref();
            let right = root.right.as_deref();

            Self::kth_largest_helper(right, out);
            out.push(root.val);
            Self::kth_largest_helper(left, out);
        }
    }
}
