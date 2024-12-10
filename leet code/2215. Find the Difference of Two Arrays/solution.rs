use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut first_hash_set = HashSet::new();
        let mut second_hash_set = HashSet::new();
        for &num in &nums1 {
            first_hash_set.insert(num);
        }

        for &num in &nums2 {
            second_hash_set.insert(num);
        }

        vec![
            first_hash_set.iter().cloned().filter(|x| !second_hash_set.contains(x)).collect::<Vec<_>>(),
            second_hash_set.iter().cloned().filter(|x| !first_hash_set.contains(x)).collect::<Vec<_>>(),
        ]
    }
}

fn sort_result(mut result: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // 對每個子數組進行排序
    result.iter_mut().for_each(|v| v.sort());
    result
}

fn main() {
    let nums1 = vec![1,2,3];
    let nums2 = vec![2,4,6];
    assert_eq!(sort_result(Solution::find_difference(nums1, nums2)), vec![vec![1,3], vec![4,6]]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_difference() {
        assert_eq!(sort_result(Solution::find_difference(vec![1,2,3], vec![2,4,6])), vec![vec![1,3], vec![4,6]]);
    }

    #[test]
    fn test_find_difference_empty() {
        assert_eq!(sort_result(Solution::find_difference(vec![1,2,3,3], vec![1,1,2,2])), vec![vec![3], vec![]]);
    }
}