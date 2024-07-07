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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(n) => {
                let mut n_ref = n.borrow_mut();
                let left = Self::invert_tree(n_ref.right.take());
                let right = Self::invert_tree(n_ref.left.take());
                n_ref.left = left;
                n_ref.right = right;
                Some(n.clone())
            }
            _ => None,
        }
    }
}

fn main() {
    // 创建一个二叉树: 4 -> 2, 7 -> 1, 3, 6, 9
    let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let node2 = Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(node1),
        right: Some(node3),
    }));
    let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
    let node9 = Rc::new(RefCell::new(TreeNode::new(9)));
    let node7 = Rc::new(RefCell::new(TreeNode {
        val: 7,
        left: Some(node6),
        right: Some(node9),
    }));
    let root = Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(node2),
        right: Some(node7),
    }));

    let result = Solution::invert_tree(Some(root));
    println!("Inverted tree: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_tree() {
        // 测试普通二叉树
        let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node2 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(node1),
            right: Some(node3),
        }));
        let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
        let node9 = Rc::new(RefCell::new(TreeNode::new(9)));
        let node7 = Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(node6),
            right: Some(node9),
        }));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(node2),
            right: Some(node7),
        }));

        let inverted = Solution::invert_tree(Some(root));
        let expected_node1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let expected_node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let expected_node2 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(expected_node3),
            right: Some(expected_node1),
        }));
        let expected_node6 = Rc::new(RefCell::new(TreeNode::new(6)));
        let expected_node9 = Rc::new(RefCell::new(TreeNode::new(9)));
        let expected_node7 = Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(expected_node9),
            right: Some(expected_node6),
        }));
        let expected_root = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(expected_node7),
            right: Some(expected_node2),
        }));

        assert_eq!(inverted, Some(expected_root));

        // 测试单个节点
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        assert_eq!(Solution::invert_tree(Some(root.clone())), Some(root));

        // 测试空树
        let root: Option<Rc<RefCell<TreeNode>>> = None;
        assert_eq!(Solution::invert_tree(root), None);
    }
}
