## Max Number of K_Sum Pairs
tag: two pointers

先把超過目標值過濾和由小到大排序，再用左右指標，記下目前左右指標的和，等於目標值將數量+1，左右指標各移動一格，若大於目標，移動右指標往較小的值移動，否則移動左指標往較大的值移動。

### ex1

> input: nums = [1,2,3,4,5], k = 4

> output: 4

> 1. 先把超過目標值過濾和由小到大排序，得到[1,2,3]
> 2. i=0,j=2,nums[i]=1,nums[j]=3,sum=1+3=4,目前左右指標的和4,4=4,total+=1,i+=1,j-=1
> 3. i=j 結束回傳1

### solution1
> time complexity: O(nlogn) sort nums vector
> space complexity: O(n) create nums.len() vector for store filter nums

### solution2
> time complexity: O(nlogn) sort nums vector
> space complexity: O(1)