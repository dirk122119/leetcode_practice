struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        //allocate stack to store asteroids
        let mut stack = VecDeque::new();
        //check last astero if collision, if collision check last two astero untill 0,if false store astero id
        for astero in asteroids {
            if stack.len()==0 {
                stack.push_back(astero);
                continue
            }
            loop {
                let last:Option<i32> = stack.pop_back();
                match last {
                    Some(x) => {
                        if x*astero >0 || (astero>0 && x<0) {
                            stack.push_back(x);
                            stack.push_back(astero);
                            break
                        } else {
                            if x.abs() > astero.abs() {
                                stack.push_back(x);
                                break
                            } else if x.abs() == astero.abs() {
                                break
                            }
                        }
                    }
                    None => {
                        stack.push_back(astero);
                        break
                    }
                }
            }
        }          
            let r = stack.into_iter().collect::<Vec<i32>>();
            r
    }
}

fn main() {
    let asteroids = vec![5, 10, -5];
    let result = Solution::asteroid_collision(asteroids);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asteroid_collision() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    }

    #[test]
    fn test_asteroid_collision_2() {
        assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
    }

    #[test]
    fn test_asteroid_collision_3() {
        assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
    }

    #[test]
    fn test_asteroid_collision_4() {
        assert_eq!(Solution::asteroid_collision(vec![1, -2, -2, 1]), vec![-2, -2, 1]);
    }
}
