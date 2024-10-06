struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut i = 0 as usize;
        let mut j = height.len()-1 as usize;
        while i<j {
            if height[i]<height[j] { 
                let volume = (j-i) as i32 * height[i];
                max = max.max(volume);
                i+=1;
            } else {
                let volume = (j-i) as i32 * height[j];
                max = max.max(volume);
                j-=1;
            }
        }
        max
    }
}

fn main() {
    assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
}