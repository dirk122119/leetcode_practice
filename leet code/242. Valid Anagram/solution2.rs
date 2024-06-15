use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // create two HashMap and compare O(n)
        let mut map_s = HashMap::new();
        let mut map_t = HashMap::new();

        for i in s.chars() {
            map_s.entry(i).and_modify(|v| *v += 1).or_insert(1);
        }
        for i in t.chars() {
            map_t.entry(i).and_modify(|v| *v += 1).or_insert(1);
        }
        map_s == map_t
    }
}

fn main() {
    let s = String::from("anagram");
    let t = String::from("nagaram");
    let result = Solution::is_anagram(s, t);
    println!("Is anagram: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(Solution::is_anagram(String::from("anagram"), String::from("nagaram")), true);
        assert_eq!(Solution::is_anagram(String::from("rat"), String::from("car")), false);
        assert_eq!(Solution::is_anagram(String::from(""), String::from("")), true);
        assert_eq!(Solution::is_anagram(String::from("a"), String::from("a")), true);
        assert_eq!(Solution::is_anagram(String::from("a"), String::from("b")), false);
    }

    #[test]
    fn test_is_anagram_case_sensitive() {
        assert_eq!(Solution::is_anagram(String::from("Hello"), String::from("hello")), false);
    }
}
