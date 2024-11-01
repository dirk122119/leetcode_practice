struct Solution1;

impl Solution1 {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase());
        let mut left = chars.next();
        let mut right = chars.next_back();
        while let (Some(l), Some(r)) = (left, right) {
            if l != r {
                return false;
            }
            left = chars.next();
            right = chars.next_back();
        }
        true
    }
}

fn main() {
    let s = "A man, a plan, a canal: Panama".to_string();
    let result = Solution1::is_palindrome(s);
    println!("Is palindrome: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_basic() {
        assert_eq!(Solution1::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
    }

    #[test]
    fn test_palindrome_empty() {
        assert_eq!(Solution1::is_palindrome("".to_string()), true);
    }

    #[test]
    fn test_palindrome_special_chars() {
        assert_eq!(Solution1::is_palindrome("race a car".to_string()), false);
    }
}
