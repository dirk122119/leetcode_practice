## Container With Most Water
tag: two pointers

我們用左右指標，移動比較短的邊界，記下目前最大的值，等全部找完，回傳最大值。

### ex1

> input: height = [1,8,6,2,5,4,8,3,7]

> output: 49

> 1. i=0,j=8,height[i]=1,height[j]=7,total=8,目前最大值8,1<7,i+=1
> 2. i=1,j=8,height[i]=8,height[j]=7,total=49,目前最大值49,8>7,j-=1
> 3. i=1,j=7,height[i]=8,height[j]=3,total=18,目前最大值49,8>3,j-=1
> 4. i=1,j=6,height[i]=8,height[j]=8,total=40,目前最大值49,8>=8,j-=1
> 5. i=1,j=5,height[i]=8,height[j]=4,total=16,目前最大值49,8>4,j-=1
> 6. i=1,j=4,height[i]=8,height[j]=5,total=15,目前最大值49,8>5,j-=1
> 7. i=1,j=3,height[i]=8,height[j]=2,total=4,目前最大值49,8>2,j-=1
> 8. i=1,j=2,height[i]=8,height[j]=6,total=6,目前最大值49,8>6,j-=1
> 9. i = j 結束回傳最大值49

### solution1
> time complexity: O(n) iterator height.len()
> space complexity: O(1) 