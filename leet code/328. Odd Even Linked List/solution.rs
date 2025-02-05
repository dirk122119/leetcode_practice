struct Solution;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head
        }
        //先分奇數linklist和偶數linklist最後把奇數linklist.next指向偶數的頭
        let mut odd_list_head = head.take().unwrap();
        let mut even_list_head = odd_list_head.next.take().unwrap();

        let mut odd_list_tail = &mut odd_list_head;
        let mut even_list_tail = &mut even_list_head;

        let mut current = even_list_tail.next.take();
        //start from third node
        let mut is_odd = true;

        while let Some(mut node) = current {
            current = node.next.take();
            if is_odd {
                odd_list_tail.next = Some(node);
                odd_list_tail = odd_list_tail.next.as_mut().unwrap();
            } else {
                even_list_tail.next = Some(node);
                even_list_tail = even_list_tail.next.as_mut().unwrap();
            }
            is_odd = !is_odd;
        }
        odd_list_tail.next = Some(even_list_head);
        return Some(odd_list_head);
    }
}

// 幫助建立測試用的鏈表的輔助函數
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
    // 測試用例 1: [1,2,3,4,5]
    let list1 = create_linked_list(vec![1,2,3,4,5]);
    let result1 = Solution::odd_even_list(list1);
    println!("Test 1: {:?}", linked_list_to_vec(result1)); // 預期輸出: [1,3,5,2,4]
    
    // 測試用例 2: [2,1,3,5,6,4,7]
    let list2 = create_linked_list(vec![2,1,3,5,6,4,7]);
    let result2 = Solution::odd_even_list(list2);
    println!("Test 2: {:?}", linked_list_to_vec(result2)); // 預期輸出: [2,3,6,7,1,5,4]
    
    // 測試用例 3: 空鏈表
    let list3 = None;
    let result3 = Solution::odd_even_list(list3);
    println!("Test 3: {:?}", linked_list_to_vec(result3)); // 預期輸出: []
    
    // 測試用例 4: 單節點鏈表
    let list4 = create_linked_list(vec![1]);
    let result4 = Solution::odd_even_list(list4);
    println!("Test 4: {:?}", linked_list_to_vec(result4)); // 預期輸出: [1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_even_list() {
        // 測試用例 1
        let input1 = create_linked_list(vec![1,2,3,4,5]);
        let result1 = Solution::odd_even_list(input1);
        assert_eq!(linked_list_to_vec(result1), vec![1,3,5,2,4]);
        
        // 測試用例 2
        let input2 = create_linked_list(vec![2,1,3,5,6,4,7]);
        let result2 = Solution::odd_even_list(input2);
        assert_eq!(linked_list_to_vec(result2), vec![2,3,6,7,1,5,4]);
    }

    #[test]
    fn test_non_odd_even_list() {
        // 測試空鏈表
        let input3 = None;
        let result3 = Solution::odd_even_list(input3);
        assert_eq!(linked_list_to_vec(result3), vec![]);
    }

    #[test]
    fn test_single_node_list() {
        let input1 = create_linked_list(vec![1]);
        let result1 = Solution::odd_even_list(input1);
        assert_eq!(linked_list_to_vec(result1), vec![1]);
    }
    
}