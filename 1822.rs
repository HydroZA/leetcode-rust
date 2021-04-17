/*
There is a function signFunc(x) that returns:

    1 if x is positive.
    -1 if x is negative.
    0 if x is equal to 0.

You are given an integer array nums. Let product be the product of all values in the array nums.

Return signFunc(product).

Runtime: 0 ms, faster than 100.00% of Rust online submissions for Sign of the Product of an Array.
Memory Usage: 2.1 MB, less than 72.41% of Rust online submissions for Sign of the Product of an Array.
*/
impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 
    {
        if nums.contains(&0)
        {
            return 0;
        }
        else 
        {
            // count number of negative numbers
            let negative_count: usize = nums.iter().filter(|x| x.is_negative()).count();
            
            if negative_count % 2 == 0
            {
                return 1;
            }
            else
            {
                return -1;
            }
        }
    }
}