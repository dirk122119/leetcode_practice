struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut i = 0;
        let mut j = 0;
        let s = s.chars().collect::<Vec<char>>();
        let t = t.chars().collect::<Vec<char>>();
        if s.len()==0 {
            return true;
        }

        while j < t.len() {
            if s[i]==t[j] {
                i+=1;
            }
            if i==s.len() {
                return true
            }
            j+=1
        }
        return false;
    }
}

fn main() {
    let s = String::from("abc");
    let t = String::from("ahbgdc");
    let result = Solution::is_subsequence(s, t);
    println!("Is subsequence: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subsequence() {
        assert_eq!(Solution::is_subsequence(String::from("abc"), String::from("ahbgdc")), true);
        assert_eq!(Solution::is_subsequence(String::from("axc"), String::from("ahbgdc")), false);
        assert_eq!(Solution::is_subsequence(String::from("axc"), String::from("ahbgdc")), false);
        assert_eq!(Solution::is_subsequence(String::from(""), String::from("a")), true);
        assert_eq!(Solution::is_subsequence(String::from("a"), String::from("")), false);
        assert_eq!(Solution::is_subsequence(String::from("a"), String::from("a")), true);
        assert_eq!(Solution::is_subsequence(String::from("ab"), String::from("a")), false);
    }
}