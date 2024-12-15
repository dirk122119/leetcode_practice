struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        //hahmap store key:row vector,value frequence
        let mut row_hashmap = HashMap::new();
        for r in &grid {
            row_hashmap.entry(r).and_modify(|x| *x+=1).or_insert(1);
        }
        //check column vec isDuplicate if true returnvalue = return value + frequency in hashmap
        let n = grid[0].len();
        let mut count = 0;
        for r in 0..n {
            let mut column_vec = vec![];
            for c in 0..n {
                column_vec.push(grid[c][r]);
            }
            if let Some(freq) = row_hashmap.get(&column_vec) {
                count += freq;
            }
        }
        count
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

    #[test]
    fn test_equal_pairs_3() {
        assert_eq!(Solution::equal_pairs(vec![vec![11,1],vec![1,11]]), 2);
    }
}
