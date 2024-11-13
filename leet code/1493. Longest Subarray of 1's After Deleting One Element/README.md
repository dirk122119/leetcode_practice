# 1493. Longest Subarray of 1's After Deleting One Element

[LeetCode Link](https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/)

## 問題描述
給定一個二進制數組 `nums`，你必須從數組中刪除一個元素。

返回刪除一個元素後，數組中最長的只包含 1 的非空子數組的長度。

如果數組中不存在這樣的子數組，返回 0。

## 解題思路
使用滑動窗口（Sliding Window）技術：
1. 維護一個窗口，允許窗口內最多有一個 0（因為必須刪除一個元素）
2. 使用 `zero_count` 記錄窗口內 0 的數量
3. 當窗口內的 0 超過 1 個時，移動左指針直到窗口內只有 1 個 0
4. 持續更新最大長度（注意：因為必須刪除一個元素，所以長度是 right - left）

### 複雜度分析
- 時間複雜度：O(n)，其中 n 是數組長度
- 空間複雜度：O(1)，只使用了常數額外空間

## 範例