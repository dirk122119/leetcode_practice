struct Solution2;

impl Solution2 {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let chars = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<Vec<char>>();
        let mut left = 0;
        let mut right = chars.len() - 1;
        while left < right {
            if chars[left] != chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

fn main() {
    let s = "A man, a plan, a canal: Panama".to_string();
    let result = Solution2::is_palindrome(s);
    println!("Is palindrome: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_basic() {
        assert_eq!(Solution2::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
    }

    #[test]
    fn test_palindrome_empty() {
        assert_eq!(Solution2::is_palindrome("".to_string()), true);
    }

    #[test]
    fn test_palindrome_special_chars() {
        assert_eq!(Solution2::is_palindrome("race a car".to_string()), false);
    }
}
