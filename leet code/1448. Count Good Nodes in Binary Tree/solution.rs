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
      right: None
    }
  }
}

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //function to get good node number
        let left = Self::count_nodes(root.as_ref().unwrap().borrow().left.clone(),root.as_ref().unwrap().borrow().val);
        let right = Self::count_nodes(root.as_ref().unwrap().borrow().right.clone(),root.as_ref().unwrap().borrow().val);
        return left+right+1
    }

    fn count_nodes(root:Option<Rc<RefCell<TreeNode>>>,mut max:i32) -> i32 {
        let mut great=0;
        if root.is_none() {
            return 0
        }
        if root.as_ref().unwrap().borrow().val >= max {
            max = root.as_ref().unwrap().borrow().val;
            great = 1;
        }
        let left = Self::count_nodes(root.as_ref().unwrap().borrow().left.clone(),max);
        let right = Self::count_nodes(root.as_ref().unwrap().borrow().right.clone(),max);
        // println!("{:?}",root.as_ref().unwrap().borrow().val);
        // println("{:?}",left+right+great);
        return left+right+great
    }
}

fn main(){
    let mut root = TreeNode::new(3);
    let left = TreeNode::new(1);
    let right = TreeNode::new(4);
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    println!("{:?}",Solution::good_nodes(Some(Rc::new(RefCell::new(root)))));
}

mod tests {
    use super::*;

    #[test]
    fn test_good_nodes() {
        let mut root = TreeNode::new(3);
        let left = TreeNode::new(1);
        let right = TreeNode::new(4);
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));
        assert_eq!(Solution::good_nodes(Some(Rc::new(RefCell::new(root)))), 2);
    }
    
    #[test]
    fn test_good_nodes_case1() {
        // 構建樹 [3,1,4,3,null,1,5]
        let mut root = TreeNode::new(3);
        let mut left = TreeNode::new(1);
        let mut right = TreeNode::new(4);
        
        // 第三層
        let left_left = TreeNode::new(3);
        let right_left = TreeNode::new(1);
        let right_right = TreeNode::new(5);
        
        // 連接第三層到第二層
        left.left = Some(Rc::new(RefCell::new(left_left)));
        right.left = Some(Rc::new(RefCell::new(right_left)));
        right.right = Some(Rc::new(RefCell::new(right_right)));
        
        // 連接第二層到根節點
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));
        
        assert_eq!(Solution::good_nodes(Some(Rc::new(RefCell::new(root)))), 4);
    }
    
    #[test]
    fn test_good_nodes_case2() {
        // 構建樹 [3,3,null,4,2]
        let mut root = TreeNode::new(3);
        let mut left = TreeNode::new(3);
        
        // 第三層
        let left_left = TreeNode::new(4);
        let left_right = TreeNode::new(2);
        
        // 連接第三層到第二層
        left.left = Some(Rc::new(RefCell::new(left_left)));
        left.right = Some(Rc::new(RefCell::new(left_right)));
        
        // 連接第二層到根節點
        root.left = Some(Rc::new(RefCell::new(left)));
        
        assert_eq!(Solution::good_nodes(Some(Rc::new(RefCell::new(root)))), 3);
    }
    
    #[test]
    fn test_good_nodes_case3() {
        // 構建樹 [1]
        let root = TreeNode::new(1);
        assert_eq!(Solution::good_nodes(Some(Rc::new(RefCell::new(root)))), 1);
    }
}


