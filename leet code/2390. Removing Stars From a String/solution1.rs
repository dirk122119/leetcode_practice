struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut return_vec = vec![];
        for i in s.bytes() {
            if i ==b'*' {
                return_vec.pop();
            } else {
                return_vec.push(i);
            }
        }
        String::from_utf8_lossy(&return_vec).to_string()
    }
}

fn main() {
    println!("{}", Solution::remove_stars("leet**cod*e".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_stars() {
        assert_eq!(Solution::remove_stars("leet**cod*e".to_string()), "lecoe");
    }

    #[test]
    fn test_remove_stars_2() {
        assert_eq!(Solution::remove_stars("erase*****".to_string()), "");
    }

    #[test]
    fn test_remove_stars_3() {
        assert_eq!(Solution::remove_stars("a*bc*d".to_string()), "bd");
    }
}