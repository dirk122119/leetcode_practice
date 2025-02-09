use std::collections::VecDeque;
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
        let mut stack_1 = VecDeque::new();
        let mut stack_2 = VecDeque::new();

        while head.is_some() {
            stack_1.push_back(head.as_ref().unwrap().val);
            head = head.unwrap().next.take();
        }
        let n = stack_1.len()/2;
        let mut count = n;

        while count>0 {
            stack_2.push_back(stack_1.pop_back().unwrap());
            count-=1;
        }

        let mut max = 0;
        while !stack_1.is_empty() {
            let temp =stack_1.pop_back().unwrap()+stack_2.pop_back().unwrap();
            if temp > max {
                max = temp
            }
        }
        max as i32
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
