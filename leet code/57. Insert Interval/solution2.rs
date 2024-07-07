use std::cmp;

struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut re = vec![];
        let n = intervals.len();
        let mut i = 0;

        //1. comapre no interopt intervals[i] < new_interval 
        while i < n && intervals[i][1] < new_interval[0] {
            re.push(intervals[i].clone());
            i += 1;
        }

         //2. comapre  interopt intervals[i] and new_interval[1]
        while i < n && intervals[i][0] <= new_interval[1] {
            new_interval[0] = cmp::min(new_interval[0], intervals[i][0]);
            new_interval[1] = cmp::max(new_interval[1], intervals[i][1]);
            i += 1;
        }
        re.push(new_interval);

        //3. comapre no interopt intervals[i] > new_interval 
        while i < n {
            re.push(intervals[i].clone());
            i += 1;
        }

        re
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
