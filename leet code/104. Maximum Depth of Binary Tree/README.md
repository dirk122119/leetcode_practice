# 104. Maximum Depth of Binary Tree

## 問題描述
給定一棵二元樹，計算其最大深度。

## 解題思路

利用遞迴的方法，對每個節點進行以下處理：  
- 如果當前節點為 `None`（空節點），則返回 0。
- 否則，分別計算左子樹和右子樹的深度。
- 最終返回較大樹深加上當前節點（即 `max(left_depth, right_depth) + 1`）作為最終結果。