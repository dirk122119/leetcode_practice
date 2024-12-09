# 1207. Unique Number of Occurrences

[LeetCode 連結](https://leetcode.com/problems/unique-number-of-occurrences/)

## 題目描述

給定一個整數陣列 `arr`，請判斷陣列中每個數字出現的次數是否都是獨一無二的。

## 解題思路

1. 使用 HashMap 統計每個數字出現的次數
2. 將所有出現次數收集到一個向量中
3. 檢查向量中是否有重複的次數
   - 如果有重複的次數，返回 false
   - 如果沒有重複的次數，返回 true

## 程式碼實現

使用 Rust 實現，主要運用了以下特性：
- HashMap 用於統計次數
- Vec 用於儲存和檢查重複的出現次數

## 複雜度分析

- 時間複雜度：O(n)，其中 n 是輸入陣列的長度
- 空間複雜度：O(n)，需要使用 HashMap 儲存每個數字的出現次數