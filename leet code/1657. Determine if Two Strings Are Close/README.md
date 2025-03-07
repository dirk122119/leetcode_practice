# LeetCode 1657 - Determine if Two Strings Are Close

## 題目描述
兩個字串如果滿足以下條件，我們稱它們是 "接近的"：
1. 兩個字串可以通過交換字符的位置得到相同的字串
2. 兩個字串可以通過將一個字符的所有出現替換為另一個字符來得到相同的字串

給定兩個字串 word1 和 word2，判斷它們是否 "接近"。

## 解題思路
要判斷兩個字串是否 "接近"，需要檢查：
1. 兩個字串的長度必須相同
2. 兩個字串包含的字符集必須相同
3. 兩個字串中字符的頻率分布必須相同（可以互換）

## 解法一：HashMap 實現
使用 HashMap 記錄每個字符的出現頻率，然後比較字符集和頻率分布。
- 時間複雜度：O(n),因為有n個字符輸入到HashMap,排序因為最多26個字符，所以可以視為常數
- 空間複雜度：O(1)，因為最多為小寫字母26個

## 解法二：陣列實現
使用固定大小的陣列來記錄字符頻率，專門針對小寫字母優化。
- 時間複雜度：O(n)
- 空間複雜度：O(1)

## 解法比較
| 特性 | HashMap 解法 | 陣列解法 |
|------|-------------|----------|
| 時間複雜度 | O(n log n) | O(n) |
| 空間複雜度 | O(k) | O(1) |
| 適用範圍 | 任意字符集 | 僅小寫字母 |
| 程式碼可讀性 | 較好 | 較差 |
| 執行效率 | 較低 | 較高 |

## 使用建議
- 如果確定輸入只包含小寫字母，建議使用陣列解法
- 如果需要處理更通用的字符集，使用 HashMap 解法