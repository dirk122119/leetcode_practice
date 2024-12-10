# 2215. 找出兩數組的不同
[LeetCode 2215. Find the Difference of Two Arrays](https://leetcode.com/problems/find-the-difference-of-two-arrays/)

## 題目描述
給定兩個整數數組 `nums1` 和 `nums2`，返回一個列表 `answer`，其中：
- `answer[0]` 是一個列表，包含 `nums1` 中所有**不存在**於 `nums2` 中的**不同**整數。
- `answer[1]` 是一個列表，包含 `nums2` 中所有**不存在**於 `nums1` 中的**不同**整數。

注意：列表中的整數可以按**任意**順序返回。

## 解題思路
1. 使用 HashSet 來存儲兩個數組的元素，自動去重
2. 使用 filter 方法找出在一個集合中但不在另一個集合中的元素

## 複雜度分析
- 時間複雜度：O(n + m)，其中 n 和 m 分別是兩個數組的長度
- 空間複雜度：O(n + m)
