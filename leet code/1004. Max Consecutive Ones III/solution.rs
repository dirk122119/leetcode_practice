struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut max = 0;
        let mut left = 0 as usize;

        for right in 0..nums.len() {
            if nums[right]==0 {
                while count == k{
                    if nums[left]==0 {
                        count-=1;
                    }
                    left+=1
                }
                count+=1;
            }
            max = max.max(right-left+1);
        }
        max as i32
    }
}

fn main() {
    let nums = vec![1,1,1,0,0,0,1,1,1,1,0];
    let k = 2;
    assert_eq!(Solution::longest_ones(nums, k), 6);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_longest_ones() {
        assert_eq!(Solution::longest_ones(vec![1,1,1,0,0,0,1,1,1,1,0], 2), 6);
    }

    #[test]
    fn test_longest_ones_2() {
        assert_eq!(Solution::longest_ones(vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], 3), 10);
    }
}
