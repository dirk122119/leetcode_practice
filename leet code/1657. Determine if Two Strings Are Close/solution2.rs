struct Solution;


impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
    // 長度不同直接false
    if word1.len() != word2.len() {
        return false;
    }
    let mut words1_vec = vec![0;26];
    let mut words2_vec = vec![0;26];

    for i in word1.bytes() {
        words1_vec[(i -b'a') as usize] += 1;
    }
    for i in word2.bytes() {
        words2_vec[(i -b'a') as usize] += 1;
    }

    for i in 0..26 {
        if (words1_vec[i]!=0) != (words2_vec[i]!=0) {
            return false
        }
    }
    words1_vec.sort();
    words2_vec.sort();

    words1_vec == words2_vec
    }
}

fn main() {
    assert_eq!(Solution::close_strings("abc".to_string(), "cba".to_string()), true);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_close_strings_true() {
        assert_eq!(Solution::close_strings("abc".to_string(), "cba".to_string()), true);
    }

    #[test]
    fn test_close_strings_false() {
        assert_eq!(Solution::close_strings("a".to_string(), "aa".to_string()), false);
    }

    #[test]
    fn test_close_strings_false_2() {
        assert_eq!(Solution::close_strings("cabbba".to_string(), "abbccc".to_string()), true);
    }
}
