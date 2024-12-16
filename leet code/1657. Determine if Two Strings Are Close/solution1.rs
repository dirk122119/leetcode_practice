struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut word1_map = HashMap::new();
        let mut word2_map = HashMap::new();

        if word1.len() != word2.len() {
            return false
        }

        for c in word1.bytes() {
            word1_map.entry(c).and_modify(|count| *count+=1).or_insert(1);
        }

        for c in word2.bytes() {
            word2_map.entry(c).and_modify(|count| *count+=1).or_insert(1);
        }

        let keys1: HashSet<_> = word1_map.keys().collect();
        let keys2: HashSet<_> = word2_map.keys().collect();
        let mut vals1: Vec<_> = word1_map.values().collect();
        let mut vals2: Vec<_> = word2_map.values().collect();

        vals1.sort();
        vals2.sort();

        keys1 == keys2 && vals1 == vals2
    }
}

fn main() {
    assert_eq!(Solution::close_strings("abc".to_string(), "cba".to_string()), true);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_close_strings() {
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
