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
        
        // 建立 dummy 節點，這可以簡化邊界條件（例如刪除 head）
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        // fast 用來一次跳兩步
        let mut fast = &(dummy.clone());
        // slow 將最終指向待刪除節點的前一個節點
        let mut slow = &mut dummy;
        
        // 每次迴圈中 slow 走一步、fast 走兩步
        // 當 fast 的下一個節點或再下一個節點不存在時，
        // 表示 fast 已到尾部，此時 slow 剛好指向中間節點前驅
        while fast.as_ref().unwrap().next.is_some()
                && fast.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
            // 移動 slow 指針一步
            slow = &mut slow.as_mut().unwrap().next;
            // 移動 fast 指針兩步
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        
        // 刪除中間節點：
        // slow.next 指向的即為中間節點，將其從鏈結中跳過
        slow.as_mut().unwrap().next =
            slow.as_mut().unwrap().next.as_mut().unwrap().next.take();
        
        // 返回處理後的鏈結（去除 dummy 節點）
        dummy.unwrap().next
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