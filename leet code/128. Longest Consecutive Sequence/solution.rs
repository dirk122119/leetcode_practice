use std::collections::HashSet;
use std::cmp;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        //1. use HashSet remove duplicate value O(n)
        let mut set = HashSet::new();
        let mut max_len = 0;
        for i in nums {
            set.insert(i);
        }
        //2. calculate consecutive num length
        for &val in set.iter() {
            // 如果当前值是序列的起始值
            if !set.contains(&(val - 1)) {
                let mut n = val;
                let mut len = 1;
                while set.contains(&(n + 1)) {
                    len += 1;
                    n += 1;
                }
                max_len = cmp::max(max_len, len);
            }
        }
         //3. return max length
        max_len
    }
}

fn main() {
    let nums = vec![100, 4, 200, 1, 3, 2];
    let result = Solution::longest_consecutive(nums);
    println!("Longest consecutive sequence length: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(Solution::longest_consecutive(vec![0, -1, 1, 2, -2, -3]), 6);
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
        assert_eq!(Solution::longest_consecutive(vec![1, 2, 0, 1]), 3);
        assert_eq!(Solution::longest_consecutive(vec![1]), 1);
    }
}
