struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0 as usize;
        let mut right = numbers.len() - 1 as usize;
        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum == target {
                return vec![left as i32 + 1, right as i32 + 1];
            }
            if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
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
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}