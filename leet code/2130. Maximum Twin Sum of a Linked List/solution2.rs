struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
impl Solution {
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let mut reverse_tree = head.clone();
        reverse_tree = Self::reverse(reverse_tree,None);

        let mut max =0;
        while reverse_tree.is_some() && head.is_some() {
            let node_1 = head.take().unwrap();
            let node_2 = reverse_tree.take().unwrap();
            let sum = node_1.val+node_2.val;
            if sum > max {
                max = sum;
            }
            head = node_1.next;
            reverse_tree = node_2.next;
        }
        max
    }

    fn reverse(head: Option<Box<ListNode>>,prev: Option<Box<ListNode>>)->Option<Box<ListNode>> {
        match head {
            None => {
                return prev
            },
            Some(mut node) => {
                let next = node.next.take();
                node.next = prev;
                return Self::reverse(next,Some(node))
            }
        }
    }
}

fn create_linked_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
    if arr.is_empty() {
        return None;
    }
    
    let mut head = Box::new(ListNode::new(arr[0]));
    let mut current = &mut head;
    
    for &val in arr.iter().skip(1) {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }
    
    Some(head)
}


fn main() {
    let arr = vec![1, 2, 3, 10];
    let head = create_linked_list(arr);
    let result = Solution::pair_sum(head);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_sum() {
        let arr = vec![5, 4, 2, 1];
        let head = create_linked_list(arr);
        let result = Solution::pair_sum(head);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_pair_sum_2() {
        let arr = vec![4,2,2,3];
        let head = create_linked_list(arr);
        let result = Solution::pair_sum(head);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_pair_sum_3() {
        let arr = vec![1,100000];
        let head = create_linked_list(arr);
        let result = Solution::pair_sum(head);
        assert_eq!(result, 100001);
    }
}
