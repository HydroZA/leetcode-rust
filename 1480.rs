/*
Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).

Return the running sum of nums.

@Author James Legge
Runtime: 0ms, 2MB Memory, faster than 100% of Rust solutions
*/

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut run_sum = vec![nums[0]];
        
        for (i, x) in nums.iter().enumerate()
        {
            if i == 0
            {
                continue;
            }
            run_sum.push(run_sum[i-1] + x);
        }
        
        run_sum
    }
}