struct Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();

        let mut count = 0;

        for r in 0..n {
            for c in 0..n {
                let mut vec_match = true;
                for i in 0..n {
                    if grid[r][i]!=grid[i][c] {
                        vec_match = false;
                        break;
                    }
                }
                if vec_match {
                    count+=1;
                }
            }
        }
        count as i32
    }
}

fn main() {
    println!("{}", Solution::equal_pairs(vec![vec![3,2,1],vec![1,7,6],vec![2,7,7]]));
    println!("{}", Solution::equal_pairs(vec![vec![3,1,2,2],vec![1,4,4,5],vec![2,4,2,2],vec![2,4,2,2]]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_pairs() {
        assert_eq!(Solution::equal_pairs(vec![vec![3,2,1],vec![1,7,6],vec![2,7,7]]), 1);
    }

    #[test]
    fn test_equal_pairs_2() {
        assert_eq!(Solution::equal_pairs(vec![vec![3,1,2,2],vec![1,4,4,5],vec![2,4,2,2],vec![2,4,2,2]]), 3);
    }
}
