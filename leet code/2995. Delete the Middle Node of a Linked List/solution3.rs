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
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut n = 1;
        let mut first = head.as_ref().unwrap().as_ref();

        // count how many nodes in list 
        while let Some(head) = first.next.as_ref() {
            first = head.as_ref();
            n +=1;
        }
        if n==1 {
            return None
        } else {
            // operator delete middle node
            let mut second = head.as_mut().unwrap().as_mut();
        
            for _ in 0..(n/2)-1 {
                second = second.next.as_mut().unwrap().as_mut();
            }
            second.next = second.next.as_mut().unwrap().as_mut().next.take();
        }
        head 
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

// 將鏈表轉換為向量以便於比較
fn linked_list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}
fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let head = create_linked_list(arr);
    let result = Solution::delete_middle(head);
    let vec = linked_list_to_vec(result);
    println!("{:?}", vec);

    let arr = vec![1, 2, 3, 4, 5, 6];
    let head = create_linked_list(arr);
    let result = Solution::delete_middle(head);
    let vec = linked_list_to_vec(result);
    println!("{:?}", vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_middle() {
        let arr = vec![1, 2, 3, 4, 5];
        let head = create_linked_list(arr);
        let result = Solution::delete_middle(head);
        let vec = linked_list_to_vec(result);
        assert_eq!(vec, vec![1, 2, 4, 5]);
    }

    #[test]
    fn test_delete_middle_single_node() {
        let head = create_linked_list(vec![1]);
        let result = Solution::delete_middle(head);
        assert_eq!(result, None);
    }

    #[test]
    fn test_delete_middle_multiple_nodes() {
        let head = create_linked_list(vec![1, 2, 3, 4, 5, 6]);
        let result = Solution::delete_middle(head);
        let vec = linked_list_to_vec(result);
        assert_eq!(vec, vec![1, 2, 3, 5, 6]);
    }
}