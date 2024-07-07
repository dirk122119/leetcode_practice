
![image](./image/img.png)

### ex1

> input: word1 = "abc", word2 = "pqr"

> output: "apbqcr"

### ex2

> input: word1 = "ab", word2 = "pqrs"

> output: "apbqrs"

## 思路
使用兩個指標分別紀錄要合併字元的位置，合併完後往右走一格，直到全部都合併完成。

時間複雜度 O(m+n) word1長度m+word2長度n
空間複雜度 O(m+n) push 兩個字串的長度到rs vector