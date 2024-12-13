struct Solution;

use std::collections::HashMap;

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

        let mut vec_key1: Vec<_> = word1_map.clone().into_keys().collect::<Vec<_>>();
        let mut vec_key2: Vec<_> = word2_map.clone().into_keys().collect::<Vec<_>>();
        
        vec_key1.sort();
        vec_key2.sort();

        if vec_key1 != vec_key2 {
            return false
        } 

        let mut vec_value1: Vec<_> = word1_map.into_values().collect::<Vec<_>>();
        let mut vec_value2: Vec<_> = word2_map.into_values().collect::<Vec<_>>();
        
        vec_value1.sort();
        vec_value2.sort();

        if vec_value1 != vec_value2 {
            return false
        }

        true
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
