## Is Subsequence
tag: two pointers

我們用快慢指標，i指標表示要判斷的值，j指標表示要比較的值，如果i指標到達字串的最後，是子字串，否則不是。

### ex1

> input: s = "abc", t = "ahbgdc"

> output: true

s = ['a', 'b', 'c']
t = ['a', 'h', 'b', 'g', 'd', 'c']

1. i=0,j=0,s[i]=a,t[j]=a,i+=1,j+=1

2. i=1,j=1,s[i]=b,t[j]=h,j+=1

3. i=1,j=2,s[i]=b,t[j]=b,i+=1,j+=1

4. i=2,j=3,s[i]=c,t[j]=g,j+=1

5. i=2,j=4,s[i]=c,t[j]=d,i+=1,j+=1

6. i=3,j=5,s[i]=c,t[j]=c,i+=1,j+=1

7. i=s.len(),return true

### ex2

> input: s = "axc", t = "ahbgdc"

> output: false

s = ['a', 'x', 'c']
t = ['a', 'h', 'b', 'g', 'd', 'c']

1. i=0,j=0,s[i]=a,t[j]=a,i+=1,j+=1

2. i=1,j=1,s[i]=x,t[j]=h,j+=1

3. i=1,j=2,s[i]=x,t[j]=b,j+=1

4. i=1,j=3,s[i]=x,t[j]=g,j+=1

5. i=1,j=4,s[i]=x,t[j]=d,j+=1

6. i=1,j=5,s[i]=x,t[j]=c,j+=1

7. j=t.len(),return false

### solution1
> time complexity: O(n) iterator t.chars()
> space complexity: O(n+m) create t.chars() and s.chars() vectors

### solution2
> time complexity: O(n) iterator t.chars()
> space complexity: O(1) without create t.chars() and s.chars() vectors