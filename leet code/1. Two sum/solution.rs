use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            if let Some(pair) = map.get(&(target - num)) {
                return vec![*pair as i32, index as i32];
            }
            map.insert(num, index as i32);
        }
        unreachable!();
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    #[should_panic]
    fn test_no_solution() {
        Solution::two_sum(vec![1, 2, 3], 7);
    }
}
