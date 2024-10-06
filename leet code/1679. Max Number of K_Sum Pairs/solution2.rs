struct Solution;

impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable(); // 原地排序
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let mut count = 0;

        while left < right {
            let target = nums[left as usize] + nums[right as usize];
            if target == k {
                count += 1;
                left += 1;
                right -= 1;
            } else if target < k {
                left += 1;
            } else {
                right -= 1;
            }
        }

        count
    }
}

fn main() {
    assert_eq!(Solution::max_operations(vec![1, 2, 3, 4, 5], 5), 2);
    println!(
        "Solution::max_operations(vec![1,2,3,4,5], 4): {}",
        Solution::max_operations(vec![1, 2, 3, 4, 5], 5)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_operations() {
        assert_eq!(Solution::max_operations(vec![1, 2, 3, 4, 5], 5), 2);
        assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
    }
}