use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut hashmap = HashMap::new();
        for item in arr {
            if let Some(x) = hashmap.get_mut(&item) {
                *x +=1;
            } else {
                hashmap.insert(item,1);
            }
        }
        let mut key_array = Vec::new();
        for (_k,v) in hashmap {
            if key_array.contains(&v) {
                return false;
            } else {
                key_array.push(v);
            }
        }
       return true;
    }
}

fn main() {
    let arr = vec![1,2,2,1,1,3];
    assert_eq!(Solution::unique_occurrences(arr), true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_occurrences() {
        assert_eq!(Solution::unique_occurrences(vec![1,2,2,1,1,3]), true);
    }

    #[test]
    fn test_unique_occurrences_false() {
        assert_eq!(Solution::unique_occurrences(vec![1,2]), false);
    }

    #[test]
    fn test_unique_occurrences_true() {
        assert_eq!(Solution::unique_occurrences(vec![-3,0,1,-3,1,1,1,-3,10,0]), true);
    }
}
