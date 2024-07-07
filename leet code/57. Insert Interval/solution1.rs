use std::cmp;

struct Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
        let re = Self::merge(intervals);
        re
    }
        pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by_key(|k| k[0]);
        let mut re_vec = vec![];
        let mut l = intervals[0][0];
        let mut r = intervals[0][1];
        for i in 1..intervals.len(){
            let s = intervals[i][0];
            let e = intervals[i][1];
            if s<=r {
                l = cmp::min(l,s);
                r = cmp::max(r,e);
            } else{
                re_vec.push(vec![l,r]);
                l = s;
                r = e;
            }
        }
        re_vec.push(vec![l,r]);
        re_vec
    }
}

fn main() {
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];
    let result = Solution::insert(intervals, new_interval);
    println!("Inserted intervals: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        assert_eq!(Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]), vec![vec![1, 5], vec![6, 9]]);
        assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![4, 8]), vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
        assert_eq!(Solution::insert(vec![], vec![5, 7]), vec![vec![5, 7]]);
        assert_eq!(Solution::insert(vec![vec![1, 5]], vec![2, 3]), vec![vec![1, 5]]);
        assert_eq!(Solution::insert(vec![vec![1, 5]], vec![2, 7]), vec![vec![1, 7]]);
    }
}
