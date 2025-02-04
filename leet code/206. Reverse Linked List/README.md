# 206. Reverse Linked List (反轉鏈表)

## 問題描述
給定一個單向鏈表的頭節點 `head`，請將該鏈表反轉並返回反轉後的鏈表。

## 示例 
輸入：head = [1,2,3,4,5]
輸出：[5,4,3,2,1]

## 解決方案

### 方案1：迭代法 (solution1.rs)
- 時間複雜度：O(n)
- 空間複雜度：O(1)

使用三個指針（prev_node、current、next_node）迭代遍歷鏈表，逐個節點改變其指向。

主要步驟：
1. 初始化一個空的前驅節點 `prev_node`
2. 遍歷原鏈表的每個節點 `current`
3. 保存當前節點的下一個節點 `next_node = current.next.take()`
4. 將當前節點指向前驅節點 `current.next = prev_node`
5. 更新前驅節點為當前節點 `prev_node = Some(current)`
6. 移動到保存的下一個節點 `head = next_node`

### 方案2：遞迴法 (solution2.rs)
- 時間複雜度：O(n)
- 空間複雜度：O(n)（Recursive Call Stack的空間）

通過遞迴方式，從鏈表的尾部開始，逐步向前反轉指針方向。

主要步驟：
1. 遞迴到鏈表末尾
2. 在遞迴過程中，當前節點的下一個節點為下一次遞迴的頭節點
3. 將當前節點的 next 指針指向上一個節點
4. 返回新的頭節點

## 延伸為何無法像 Python 一樣直接反轉

在 Rust 中，由於所有權和借用規則的限制，我們不能像 Python 那樣直接修改指針。主要原因是：

1. Rust 中的 `Box<T>` 擁有其內容的所有權
2. 需要使用 `take()` 方法來獲取並轉移所有權
3. 必須明確處理所有權的轉移

Python 版本：
```python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
    
        if head==None or head.next==None :
            return head
        
        pre = self.reverseList(head.next)
        head.next.next = head
        head.next = None
        return pre
```

Rust 中需要使用 `take()` 方法來安全地處理所有權轉移。