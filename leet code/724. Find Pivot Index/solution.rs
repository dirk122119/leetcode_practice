struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        //get sum of all numbers
        let mut right = 0 as i32;
        let mut left = 0 as i32;
        let mut pivot = -1 as i32;
        for i in 0..nums.len() {
            right+=nums[i];
        }
        //loop to check is left sum equal right sum while i is pivot
        for i in 0..nums.len() {
            right-=nums[i];
            if i>0 {
                left+=nums[i-1];
            }
            if left==right {
                pivot = i as i32;
                break
            }
        }
        pivot
    }
}

fn main() {
    let nums = vec![1,7,3,6,5,6];
    assert_eq!(Solution::pivot_index(nums), 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot_index() {
        assert_eq!(Solution::pivot_index(vec![1,7,3,6,5,6]), 3);
    }

    #[test]
    fn test_pivot_index_2() {
        assert_eq!(Solution::pivot_index(vec![1,2,3]), -1);
    }

    #[test]
    fn test_pivot_index_3() {
        assert_eq!(Solution::pivot_index(vec![2,1,-1]), 0);
    }
}
