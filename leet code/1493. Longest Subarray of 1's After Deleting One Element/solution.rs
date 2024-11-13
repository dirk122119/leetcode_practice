struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut zero_count = 0;
        let mut left = 0 as usize;
        let mut max =0;
        for right in 0..nums.len() {
            if nums[right]==0 {
                zero_count+=1;
                while zero_count > 1 {
                    if nums[left]==0 {
                        zero_count-=1;
                    }
                    left+=1;
                }
            }
            max = max.max(right - left)
        }

        max as i32
    }
}

fn main() {
    let nums = vec![1,1,0,1];
    assert_eq!(Solution::longest_subarray(nums), 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_subarray() {
        assert_eq!(Solution::longest_subarray(vec![1,1,0,1]), 3);
    }

    #[test]
    fn test_longest_subarray_2() {
        assert_eq!(Solution::longest_subarray(vec![0,1,1,1,0,1,1,0,1]), 5);
    }

    #[test]
    fn test_longest_subarray_3() {
        assert_eq!(Solution::longest_subarray(vec![1,1,1]), 2);
    }
}
