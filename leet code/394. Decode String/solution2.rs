use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut counts_stack = VecDeque::new();
        let mut string_stack = VecDeque::new();
        let mut current_string = String::new();
        let mut count = 0;

        for c in s.chars() {
            if c.is_ascii_digit() {
                count = count*10+(c as i32 -'0' as i32);
            } else if c =='[' {
                counts_stack.push_back(count);
                string_stack.push_back(current_string);
                count = 0;
                current_string = String::new();
            } else if c ==']' {
                let k:Option<i32> = counts_stack.pop_back();
                let repeated_string= String::new();
                match k {
                    Some(k) => {
                        current_string=current_string.repeat(k as usize);
                    },
                    _=>{}
                }
                let s:Option<String> = string_stack.pop_back();
                match s {
                    Some(s) => {
                        current_string=s+&current_string;
                    },
                    _=>{current_string =current_string;}
                }
                
            } else {
                current_string = current_string+&c.to_string();
            }
        }
        current_string
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