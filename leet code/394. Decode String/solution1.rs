// TODO README
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = VecDeque::new();
        let mut current_string = String::new();
        for c in s.chars() {
            if c!= ']' {
                stack.push_back(c);
                continue
             }
             if c==']' {
                loop {
                    let pop_c:Option<char> = stack.pop_back();
                    match pop_c {
                        Some(pop_c) => {
                            if pop_c == '[' {
                                break
                            }
                            current_string.insert(0, pop_c);
                        }
                        _ => {break}
                    }
                }
                let mut repeat_times = String::new();
                loop {
                    let pop_num:Option<char> = stack.pop_back();
                    match pop_num {
                        Some(pop_num) => {
                            if !pop_num.is_ascii_digit() {
                                stack.push_back(pop_num);
                                break
                            }
                            repeat_times.insert(0,pop_num);
                        }
                        _ => {break}
                    } 
                }
                if repeat_times.is_empty() {

                } else {
                    let k:usize = repeat_times.parse().unwrap();
                    current_string = current_string.repeat(k as usize); 
                }

                for ch in current_string.chars(){
                    stack.push_back(ch)
                }
                repeat_times.clear();
                current_string.clear();
                
             }
        }
        current_string = stack.into_iter().collect();
        return current_string
    }
}

fn main() {
    let s = String::from("3[a]2[bc]");
    let result = Solution::decode_string(s);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_string() {
        let s = String::from("3[a]2[bc]");
        let result = Solution::decode_string(s);
        assert_eq!(result, "aaabcbc");
    }

    #[test]
    fn test_decode_string2() {
        let s = String::from("3[a2[c]]");
        let result = Solution::decode_string(s);
        assert_eq!(result, "accaccacc");
    }

    #[test]
    fn test_decode_string3() {
        let s = String::from("2[abc]3[cd]ef");
        let result = Solution::decode_string(s);
        assert_eq!(result, "abcabccdcdcdef");
    }
}
