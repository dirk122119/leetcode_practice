## Two Sum II - Input Array Is Sorted
tag: two pointers

題目給定由小到大有排序的陣列，且一定只有兩個值符合。

作法：用左右指標判斷目前指標值之和是否為target，如果為target就返回左右的index，和大於target，移動右指標往和小的方向找，和小於target，移動左指標往和大的方向找。

solution
> time complexity: O(n)

> space complexity: O(1)