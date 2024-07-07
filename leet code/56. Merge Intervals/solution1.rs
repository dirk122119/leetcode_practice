use std::cmp;

struct Solution;

impl Solution {
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
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let result = Solution::merge(intervals);
    println!("Merged intervals: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        assert_eq!(Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]), vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
        assert_eq!(Solution::merge(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);
        assert_eq!(Solution::merge(vec![vec![1, 4], vec![2, 3]]), vec![vec![1, 4]]);
        assert_eq!(Solution::merge(vec![vec![1, 4], vec![0, 4]]), vec![vec![0, 4]]);
        assert_eq!(Solution::merge(vec![vec![1, 4], vec![2, 5], vec![7, 9], vec![8, 10]]), vec![vec![1, 5], vec![7, 10]]);
    }
}
