# 1004. Max Consecutive Ones III

[LeetCode Link](https://leetcode.com/problems/max-consecutive-ones-iii/)

## 問題描述
給定一個二進制數組 `nums` 和一個整數 `k`，你可以最多將 `k` 個 0 翻轉成 1。返回僅包含 1 的最長子數組的長度。

## 解題思路
使用滑動窗口（Sliding Window）技術：
1. 維護一個窗口，記錄當前已經翻轉的 0 的數量（count）
2. 右指針不斷向右移動，遇到 0 時增加計數
3. 當要翻轉的 0 大於 k 時，左指針向右移動，直到窗口內的 0 數量符合要求
4. 持續更新最大長度

時間複雜度：O(n)
空間複雜度：O(1)