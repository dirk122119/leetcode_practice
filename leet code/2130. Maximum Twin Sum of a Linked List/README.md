# LeetCode 2130 - 鏈表最大孿生和

在這個專案中，我們提供了兩種不同的解法來解決 LeetCode 題目「鏈表最大孿生和」（Maximum Twin Sum of a Linked List），使用 Rust 語言實作。

## 題目描述

給定一個偶數節點的鏈表，其中節點按順序配對：第一個節點和最後一個節點是一對，第二個節點和倒數第二個節點是一對，以此類推。

對於每一對節點，我們需要計算它們的和，並找出所有配對中的最大和。

## 解法介紹

本專案提供兩種解法，各自有不同的實作方式與考量：

- **解法 1 (`solution1.rs`)**  
  使用雙端隊列（VecDeque）的方式：
  - 先將所有節點的值存入第一個隊列中。
  - 將後半部分的節點移到第二個隊列中。
  - 同時從兩個隊列取出值並計算和，找出最大值。
  > 優點：實作直觀，易於理解；缺點：需要額外的空間存儲節點值。

- **解法 2 (`solution2.rs`)**  
  使用克隆和反轉的方式：
  - 先複製一份原始鏈表。
  - 將複製的鏈表進行反轉。
  - 同時遍歷原始鏈表和反轉後的鏈表，計算對應節點值的和，找出最大值。
  > 優點：實作邏輯清晰，不需要計算長度；缺點：需要額外空間存儲克隆的鏈表。
<!-- TODO: 解法3 -->
- **解法 3 (`solution3.rs`)**
  優化solution2，使用leetcode 2095方法用快慢指針找到中點，然後反轉後半部分，再同時遍歷原始鏈表和反轉後的鏈表，計算對應節點值的和，找出最大值。

## 複雜度分析

兩種解法的時間複雜度都是 O(n)，其中 n 是鏈表的長度。

空間複雜度：
- 解法 1：O(n)，需要額外的隊列存儲節點值
- 解法 2：O(n)，需要額外空間存儲克隆的鏈表

## 測試案例

兩個解法都包含了相同的測試案例，確保程式的正確性：
1. 測試基本案例：[5,4,2,1] -> 6
2. 測試相同值案例：[4,2,2,3] -> 7
3. 測試極端值案例：[1,100000] -> 100001