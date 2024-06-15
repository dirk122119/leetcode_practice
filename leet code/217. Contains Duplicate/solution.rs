use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // HashSet time: O(n) space: O(n)
        let mut set = HashSet::new();
        for i in 0..nums.len() {
            match set.contains(&nums[i]){
                true => return true,
                false => set.insert(nums[i]),
            };
        }
        false
    }
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    let result = Solution::contains_duplicate(nums);
    println!("Contains duplicate: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
        assert_eq!(Solution::contains_duplicate(vec![]), false);
    }

    #[test]
    fn test_contains_duplicate_single_element() {
        assert_eq!(Solution::contains_duplicate(vec![1]), false);
    }
}
