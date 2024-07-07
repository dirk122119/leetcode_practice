use std::cmp;

struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by_key(|k| k[0]);
        let mut re = vec![];
        let mut i =0;
        //1.for loop travel every items
        while i < intervals.len(){
            let mut start = intervals[i][0];
            let mut end = intervals[i][1];
            //2.in loop check is_overlap?
            while i<intervals.len()-1 && Self::is_overlap(vec![start,end],intervals[i+1].clone()) {
                let new_area = Self::combine(vec![start,end],intervals[i+1].clone());
                start = new_area[0];
                end = new_area[1];
                i+=1;
            }
        //3.if overlap combine two items and push to return vecel, if not overlap push to return vecel
            re.push(vec![start,end]);
            i+=1
        }
        re
    }

    pub fn is_overlap(current:Vec<i32>,next:Vec<i32>)->bool{
        return next[0]<=current[1]
    }

    pub fn combine(current:Vec<i32>,next:Vec<i32>)->Vec<i32>{
        let new_start = cmp::min(current[0],next[0]);
        let new_end = cmp::max(current[1],next[1]);
        return vec![new_start,new_end]
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
