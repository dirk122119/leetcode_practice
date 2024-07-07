use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for num in nums {
            map.entry(num).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut sorted: Vec<_> = map.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));

        sorted.iter().take(k as usize).map(|(&a, _)| a).collect()
    }
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result = Solution::top_k_frequent(nums, k);
    println!("Top k frequent elements: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        let mut result = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);

        let mut result = Solution::top_k_frequent(vec![1], 1);
        result.sort();
        assert_eq!(result, vec![1]);

        let mut result = Solution::top_k_frequent(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 2);
        result.sort();
        assert_eq!(result, vec![3, 4]);

        let mut result = Solution::top_k_frequent(vec![5, 5, 5, 6, 6, 7], 1);
        result.sort();
        assert_eq!(result, vec![5]);

        let mut result = Solution::top_k_frequent(vec![4, 1, -1, 2, -1, 2, 3], 2);
        result.sort();
        assert_eq!(result, vec![-1, 2]);
    }
}
