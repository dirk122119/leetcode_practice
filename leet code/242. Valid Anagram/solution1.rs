struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // 1. compare two strings after sorting，O(nlogn)
        let mut vec_s: Vec<char> = s.chars().collect();
        let mut vec_t: Vec<char> = t.chars().collect();
        vec_s.sort();
        vec_t.sort();
        return vec_s==vec_t
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
