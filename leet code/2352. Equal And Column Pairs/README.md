# 2352. 相等的行列對
[LeetCode 2352. Equal Row and Column Pairs](https://leetcode.com/problems/equal-row-and-column-pairs/)

## 題目描述
給定一個大小為 n x n 的矩陣 grid，返回滿足以下條件的 (行, 列) 數對的數量：
- 行 i 和列 j 所包含的元素完全相同（按相同的順序）。

### 解題思路 Solution 1 (Brute Force)
- 時間複雜度：O(n³)
- 空間複雜度：O(1)
- 優點：
  - 不需要額外空間
- 缺點：
  - 時間複雜度較高

### 解題思路 Solution 2 (HashMap)
- 時間複雜度：O(n²)
- 空間複雜度：O(n²):因為要存n*n個元素在HashMap
- 優點：
  - 時間複雜度較低
  - 使用 HashMap 優化查找過程
- 缺點：
  - 需要額外空間存儲 HashMap
<!-- TODO: 解題思路 Solution 2(Trie -->
## 比較
1. 時間效率：
   - Solution 2 優於 Solution 1
   - 當 n 較大時，差異更明顯

2. 空間使用：
   - Solution 1 優於 Solution 2
   - Solution 1 只需要常數空間

3. 代碼複雜度：
   - Solution 1 較為直觀
   - Solution 2 需要理解 HashMap 的使用

4. 適用場景：
   - 小規模數據：兩種解法都可以
   - 大規模數據：建議使用 Solution 2

