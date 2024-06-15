use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // HashMap key=[1,0,0,0,0,1,.....]:Vec[26] value vec!["eat","ate","tea"]
        // O(KN)
        let mut map: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
        for i in 0..strs.len() {
            // create key 1,0,0,0,0,1,.....]
            let mut list = vec![0; 26];
            for &c_ascii in strs[i].as_bytes() {
                list[c_ascii as usize - b'a' as usize] += 1;
            }
            map.entry(list).or_insert_with(Vec::new).push(strs[i].clone());
        }
        map.into_values().collect()
    }
}

fn main() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    let result = Solution::group_anagrams(strs);
    for group in result {
        println!("{:?}", group);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let mut result = Solution::group_anagrams(input);
        for group in result.iter_mut() {
            group.sort();  // Sort to ensure the order does not affect the test
        }
        result.sort();  // Sort the outer vector for the same reason
        let mut expected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];
        for group in expected.iter_mut() {
            group.sort();
        }
        expected.sort();
        assert_eq!(result, expected);
    }
}
