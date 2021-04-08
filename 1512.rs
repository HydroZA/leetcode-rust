/*
Given an array of integers nums.

A pair (i,j) is called good if nums[i] == nums[j] and i < j.

Return the number of good pairs.

Runtime: 0 ms, faster than 100.00% of Rust online submissions for Number of Good Pairs.
Memory Usage: 2.3 MB, less than 35.48% of Rust online submissions for Number of Good Pairs.
*/
impl Solution 
{
    pub fn num_identical_pairs (nums: Vec<i32>) -> i32 
    {
        let mut pairs: i32 = 0;
        
        for (i, num1) in nums.iter().enumerate()
        {
            for (j, num2) in nums.iter().enumerate().skip(i + 1)
            {
                if num1 == num2
                {
                    pairs += 1;
                }
            }
        }
        
        pairs
    }
}