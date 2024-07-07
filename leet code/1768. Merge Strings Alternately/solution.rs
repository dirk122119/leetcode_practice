struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        // let mut rs = String::new(); runtime beats 68%
        let mut rs = String::with_capacity(word1.len() + word2.len());//runtime beats 100%
        let mut w1_ptr = 0;
        let mut w2_ptr = 0;
        loop{
            if w1_ptr<word1.len() && w2_ptr<word2.len() {
                rs.push_str(&word1[w1_ptr..w1_ptr+1]);
                rs.push_str(&word2[w2_ptr..w2_ptr+1]);
                w1_ptr +=1;
                w2_ptr +=1;
            } else if w1_ptr<word1.len() {
                rs.push_str(&word1[w1_ptr..w1_ptr+1]);
                w1_ptr +=1;
            } else if w2_ptr<word2.len(){
                rs.push_str(&word2[w2_ptr..w2_ptr+1]);
                w2_ptr +=1;
            } else{
                break;
            } 
        }
        rs
    }
}

fn main() {
    let word1 = String::from("abc");
    let word2 = String::from("pqr");
    let result = Solution::merge_alternately(word1, word2);
    println!("Merge alternately: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_alternately() {
        assert_eq!(Solution::merge_alternately(String::from("abc"), String::from("pqr")), String::from("apbqcr"));
        assert_eq!(Solution::merge_alternately(String::from("ab"), String::from("pq")), String::from("apbq"));
        assert_eq!(Solution::merge_alternately(String::from("a"), String::from("b")), String::from("ab"));
        assert_eq!(Solution::merge_alternately(String::from("ab"), String::from("c")), String::from("acb"));
        assert_eq!(Solution::merge_alternately(String::from("a"), String::from("")), String::from("a"));
        assert_eq!(Solution::merge_alternately(String::from(""), String::from("b")), String::from("b"));
    }
}