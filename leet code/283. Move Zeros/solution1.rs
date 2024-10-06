struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {

        if s.len()>t.len() {
            return false
        }

        if s.is_empty() {
            return true
        }

        let mut s_chars = s.chars();
        let mut current_char = s_chars.next();

        for t_char in t.chars() {
            if let Some(c) = current_char {
                if c == t_char {
                    current_char = s_chars.next();
                    if current_char.is_none() {
                        return true
                    }
                }
            } else {
                return true
            }
        }
        false
    }
}

fn main() {
    let mut nums = vec![0,1,0,3,12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1,3,12,0,0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0,1,0,3,12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1,3,12,0,0]);
    }
    #[test]
    fn test_move_zeroes2() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }
}