struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        //let mut rs = String::new();
        let mut rs = String::with_capacity(word1.len() + word2.len());
        let mut w1_iter = word1.chars();
        let mut w2_iter = word2.chars();
        loop{
            match(w1_iter.next(),w2_iter.next()) {
                (Some(c1),Some(c2)) => {
                    rs.push(c1);
                    rs.push(c2);
                },
                (Some(c),None) | (None,Some(c)) => rs.push(c),
                (None,None) => return rs
            }
        }
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