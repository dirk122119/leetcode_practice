# 1732. Find the Highest Altitude

[LeetCode Link](https://leetcode.com/problems/find-the-highest-altitude/)

## 問題描述
有一個騎自行車的人從海拔為 0 的點開始行進。給定一個整數數組 `gain`，其中 `gain[i]` 是點 i 和點 i + 1 的淨海拔差。

返回騎自行車的人到達的最高點的海拔。

## 解題思路
使用累加和（Prefix Sum）的概念：
1. 初始海拔為 0
2. 遍歷 gain 數組，不斷累加海拔變化
3. 在遍歷過程中記錄最高海拔

### 算法步驟
1. 設置兩個變量：
   - `current`：記錄當前海拔
   - `highest`：記錄最高海拔
2. 遍歷 gain 數組：
   - 更新當前海拔：`current += gain[i]`
   - 更新最高海拔：`highest = highest.max(current)`
3. 返回最高海拔

### 複雜度分析
- 時間複雜度：O(n)，其中 n 是數組長度
- 空間複雜度：O(1)，只使用了常數額外空間

## 範例