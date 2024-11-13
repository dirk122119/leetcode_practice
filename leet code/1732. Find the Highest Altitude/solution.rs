struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut highest = 0;
        let mut current = 0;
        for alt in 0..gain.len() {
            current+=gain[alt];
            highest=highest.max(current);
        }
        highest as i32
    }
}

fn main() {
    let gain = vec![-5,1,5,0,-7];
    assert_eq!(Solution::largest_altitude(gain), 1);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_altitude() {
        assert_eq!(Solution::largest_altitude(vec![-5,1,5,0,-7]), 1);
    }

    #[test]
    fn test_largest_altitude_2() {
        assert_eq!(Solution::largest_altitude(vec![-4,-3,-2,-1,4,3,2]), 0);
    }
}
