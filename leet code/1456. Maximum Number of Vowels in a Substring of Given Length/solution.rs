struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let bytes = s.as_bytes();
        let mut count = bytes[..k].iter().filter(|&&b| matches!(b, b'a' | b'e' | b'i' | b'o' | b'u')).count();

        let mut max = count;

        for i in k..s.len() {
            if matches!(bytes[i-k],b'a' | b'e' | b'i' | b'o' | b'u') {
                count-=1;
            }
            if matches!(bytes[i],b'a' | b'e' | b'i' | b'o' | b'u') {
                count+=1;
            }
            max = max.max(count);
        }
        max as i32
    }
}

fn main() {
    let s = "abciiidef".to_string();
    let k = 3;
    assert_eq!(Solution::max_vowels(s, k), 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_vowels() {
        let s = "abciiidef".to_string();
        let k = 3;
        assert_eq!(Solution::max_vowels(s, k), 3);
    }

    #[test]
    fn test_max_vowels2() {
        let s = "aeiou".to_string();
        let k = 2;
        assert_eq!(Solution::max_vowels(s, k), 2);
    }

    #[test]
    fn test_max_vowels3() {
        let s = "leetcode".to_string();
        let k = 3;
        assert_eq!(Solution::max_vowels(s, k), 2);
    }
}