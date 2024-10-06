struct Solution;

impl Solution {
    pub fn max_operations( nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums
            .into_iter()
            .filter(|val| val < &k)
            .collect::<Vec<i32>>();

        if nums.len() == 0 {
            return 0;
        }
        let mut i = 0 as usize;
        let mut j = nums.len() - 1 as usize;
        let mut return_num = 0;
        nums.sort();

        while i < j {
            let target = nums[i] + nums[j];
            if target < k {
                i += 1;
            } else if target > k {
                j -= 1;
            } else {
                return_num += 1;
                i += 1;
                j -= 1;
            }
        }
        return return_num;
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
