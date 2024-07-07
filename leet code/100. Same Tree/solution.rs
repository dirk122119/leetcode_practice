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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(ref node1), Some(ref node2)) => {
                if node1.borrow().val == node2.borrow().val {
                    return Self::is_same_tree(node1.borrow().left.clone(), node2.borrow().left.clone())
                        && Self::is_same_tree(node1.borrow().right.clone(), node2.borrow().right.clone());
                } else {
                    false
                }
            }
        }
    }
}

fn main() {
    let node1_p = Rc::new(RefCell::new(TreeNode::new(2)));
    let node2_p = Rc::new(RefCell::new(TreeNode::new(3)));
    let root_p = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(node1_p),
        right: Some(node2_p),
    }));

    let node1_q = Rc::new(RefCell::new(TreeNode::new(2)));
    let node2_q = Rc::new(RefCell::new(TreeNode::new(3)));
    let root_q = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(node1_q),
        right: Some(node2_q),
    }));

    let result = Solution::is_same_tree(Some(root_p), Some(root_q));
    println!("Are the trees same? {:?}", result); // 输出: true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_tree() {
        let node1_p = Rc::new(RefCell::new(TreeNode::new(2)));
        let node2_p = Rc::new(RefCell::new(TreeNode::new(3)));
        let root_p = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(node1_p),
            right: Some(node2_p),
        }));

        let node1_q = Rc::new(RefCell::new(TreeNode::new(2)));
        let node2_q = Rc::new(RefCell::new(TreeNode::new(3)));
        let root_q = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(node1_q),
            right: Some(node2_q),
        }));

        assert!(Solution::is_same_tree(Some(root_p), Some(root_q)));

        let node1_p = Rc::new(RefCell::new(TreeNode::new(2)));
        let root_p = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(node1_p),
            right: None,
        }));

        let node1_q = Rc::new(RefCell::new(TreeNode::new(2)));
        let root_q = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(node1_q),
        }));

        assert!(!Solution::is_same_tree(Some(root_p), Some(root_q)));

        let root_p: Option<Rc<RefCell<TreeNode>>> = None;
        let root_q: Option<Rc<RefCell<TreeNode>>> = None;
        assert!(Solution::is_same_tree(root_p, root_q));
    }
}
