struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        //preProduct * sufProduct
        // O(n)+O(n)
        let mut product = 1;
        let mut re_vec = vec![1;nums.len()];
        //get every item preProduct
        //before [1,2,3,4] return [1,1,1,1]
        //1. i=0 product = 1*1 return [1,1*1,1,1]
        //2. i=1 product = 1*2 return [1,1,1*2,1]
        //3. i=2 product = 2*3 return [1,1,2,1*6]
        //after  [1,2,3,4] return [1,1,2,6]
        for i in 0..nums.len()-1{
            product=product*nums[i];
            re_vec[i+1]=product;
        }
        //get every item sufProduct
        //before [1,2,3,4] return [1,1,2,6]
        //1. i=3 product = 1*4 return [1,1,2*4,6]
        //2. i=2 product = 4*3 return [1,1*12,8,6]
        //3. i=1 product = 12*1 return [1*24,12,8,6]
        //after  [1,2,3,4] return [24,12,8,6]
        product = 1;
        for i in (1..nums.len()).rev(){
            product = product*nums[i];
            re_vec[i-1] = product*re_vec[i-1];
        }
        re_vec
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let result = Solution::product_except_self(nums);
    println!("Product except self: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self() {
        assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(Solution::product_except_self(vec![4, 3, 2, 1]), vec![6, 8, 12, 24]);
        assert_eq!(Solution::product_except_self(vec![2, 3, 4, 5]), vec![60, 40, 30, 24]);
        assert_eq!(Solution::product_except_self(vec![1, 2]), vec![2, 1]);
        assert_eq!(Solution::product_except_self(vec![10, 3, 5, 6, 2]), vec![180, 600, 360, 300, 900]);
    }
}
