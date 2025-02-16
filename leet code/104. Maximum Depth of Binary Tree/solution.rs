use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(PartialEq, Eq, Clone, Debug)]
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let l_depth = Self::max_depth(node.borrow().left.clone());
                let r_depth = Self::max_depth(node.borrow().right.clone());
                return l_depth.max(r_depth)+1;
            }
            None => 0,
        }
    }
}

fn main() {
    let node15 = Rc::new(RefCell::new(TreeNode::new(15)));
    let node7 = Rc::new(RefCell::new(TreeNode::new(7)));
    let node20 = Rc::new(RefCell::new(TreeNode {
        val: 20,
        left: Some(node15),
        right: Some(node7),
    }));
    let node9 = Rc::new(RefCell::new(TreeNode::new(9)));
    let root = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(node9),
        right: Some(node20),
    }));

    let result = Solution::max_depth(Some(root));
    println!("Max depth: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth() {
        // 测试普通二叉树
        let node15 = Rc::new(RefCell::new(TreeNode::new(15)));
        let node7 = Rc::new(RefCell::new(TreeNode::new(7)));
        let node20 = Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(node15),
            right: Some(node7),
        }));
        let node9 = Rc::new(RefCell::new(TreeNode::new(9)));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(node9),
            right: Some(node20),
        }));
        assert_eq!(Solution::max_depth(Some(root)), 3);

        // 测试单个节点
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        assert_eq!(Solution::max_depth(Some(root)), 1);

        // 测试空树
        let root: Option<Rc<RefCell<TreeNode>>> = None;
        assert_eq!(Solution::max_depth(root), 0);
    }
}
