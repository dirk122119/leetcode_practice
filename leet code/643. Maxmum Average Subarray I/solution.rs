struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = 0;
        for i in 0..k {
            sum += nums[i as usize];
        }
        let mut max = sum;
        for i in k as usize..nums.len() {
            sum = sum - nums[i as usize - k as usize] + nums[i as usize];
            if (sum > max) {
                max = sum;
            }
        }
        max as f64 / k as f64
    }
}

fn main() {
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 4;
    let result = Solution::find_max_average(nums, k);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_average() {
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75
        );
        assert_eq!(Solution::find_max_average(vec![5], 1), 5.0);
        assert_eq!(Solution::find_max_average(vec![0, 1, 1, 3, 3], 4), 2.0);
    }
}
