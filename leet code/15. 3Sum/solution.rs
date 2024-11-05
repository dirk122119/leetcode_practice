struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        nums.sort();

        for i in 0..nums.len() {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        result
    }
}

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let result = Solution::three_sum(nums);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
        assert_eq!(Solution::three_sum(vec![0,1,1]), Vec::<Vec<i32>>::new());
        assert_eq!(Solution::three_sum(vec![0,0,0]), vec![vec![0,0,0]]);
    }
}