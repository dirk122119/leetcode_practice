// Definition for a binary tree node.
struct Solution;
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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // Do recursive return all leafs
        let root1_leafs = Self::get_leafs(root1);
        let root2_leafs = Self::get_leafs(root2);
        return root1_leafs == root2_leafs
    }

    pub fn get_leafs(root:Option<Rc<RefCell<TreeNode>>>)->Vec<i32>{
        if root.is_none() {
            return vec!()
        }
        let rc_node = root.unwrap(); 
        let node = rc_node.borrow();  
        if node.left.clone().is_none() && node.right.clone().is_none() {
            return vec!(node.val)
        }
        let mut left_leafs = Self::get_leafs(node.left.clone());
        let mut right_leafs = Self::get_leafs(node.right.clone());
        left_leafs.append(&mut right_leafs);
        return left_leafs
    }
}


// 建立 TreeNode 節點
fn new_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

fn main() {
    // 正確測試：建立兩顆葉節點序列相同的樹
    // 樹結構：
    //       3
    //      / \
    //     5   1
    //    / \  / \
    //   6  2 9  8
    //     / \
    //    7  4
    let node7 = new_node(7);
    let node4 = new_node(4);
    let node2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: node7,
        right: node4,
    })));
    let node6 = new_node(6);
    let node5 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: node6,
        right: node2,
    })));
    let node9 = new_node(9);
    let node8 = new_node(8);
    let node1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: node9,
        right: node8,
    })));
    let tree1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: node5,
        right: node1,
    })));

    // 正確測試：讓 tree2 與 tree1 結構完全相同
    let tree2 = tree1.clone();
    println!("正確測試 (應輸出 true): {}", Solution::leaf_similar(tree1.clone(), tree2));

    // 錯誤測試：建立另一顆樹，將 4 改為 10，其餘結構相同
    let node7_neg = new_node(7);
    let node10 = new_node(10);
    let node2_neg = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: node7_neg,
        right: node10,
    })));
    let node6_neg = new_node(6);
    let node5_neg = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: node6_neg,
        right: node2_neg,
    })));
    let node9_neg = new_node(9);
    let node8_neg = new_node(8);
    let node1_neg = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: node9_neg,
        right: node8_neg,
    })));
    let tree3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: node5_neg,
        right: node1_neg,
    })));
    // tree1=[6,7,4,9,8]，tree3=[6,7,10,9,8]
    println!("錯誤測試 (應輸出 false): {}", Solution::leaf_similar(tree1, tree3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_similar_positive() {
        let node7 = new_node(7);
        let node4 = new_node(4);
        let node2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node7,
            right: node4,
        })));
        let node6 = new_node(6);
        let node5 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: node6,
            right: node2,
        })));
        let node9 = new_node(9);
        let node8 = new_node(8);
        let node1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node9,
            right: node8,
        })));
        let tree1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: node5,
            right: node1,
        })));
        let tree2 = tree1.clone();
        assert!(Solution::leaf_similar(tree1, tree2));
    }

    #[test]
    fn test_leaf_similar_negative() {
        let node7 = new_node(7);
        let node4 = new_node(4);
        let node2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node7,
            right: node4,
        })));
        let node6 = new_node(6);
        let node5 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: node6,
            right: node2,
        })));
        let node9 = new_node(9);
        let node8 = new_node(8);
        let node1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node9,
            right: node8,
        })));
        let tree1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: node5,
            right: node1,
        })));

        let node7_neg = new_node(7);
        let node10 = new_node(10);
        let node2_neg = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node7_neg,
            right: node10,
        })));
        let node6_neg = new_node(6);
        let node5_neg = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: node6_neg,
            right: node2_neg,
        })));
        let node9_neg = new_node(9);
        let node8_neg = new_node(8);
        let node1_neg = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node9_neg,
            right: node8_neg,
        })));
        let tree2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: node5_neg,
            right: node1_neg,
        })));
        assert!(!Solution::leaf_similar(tree1, tree2));
    }
}

