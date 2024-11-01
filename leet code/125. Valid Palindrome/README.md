## Valid Palindrome
tag: two pointers

根據題目說法，移除非英文與數字的字元並轉成小寫字元，再用左右指標判斷是否是相同字元，是的話就是回文。

### ex1
Input: s = "A man, a plan, a canal: Panama"
Output: true
1. 轉成小寫字元 "amanaplanacanalpanama"
2. 左右指標判斷是否相同字元，不同就return false,全部相同就return true

### solution1
>time complexity: O(n)
>space complexity: O(1)

### solution2
>time complexity: O(n)

>space complexity: O(n) 

>因為collect()，把處理後的字元存成Vec，所以字元數量的空間複雜度是O(n)