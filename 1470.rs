/*
Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].

Return the array in the form [x1,y1,x2,y2,...,xn,yn].

Runtime: 0 ms, faster than 100.00% of Rust online submissions for Shuffle the Array.
Memory Usage: 2.2 MB, less than 9.09% of Rust online submissions for Shuffle the Array.
*/

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> 
    {
        let mut shuffled = vec![];
        for (i, x) in nums.iter().enumerate().take(n as usize)
        {
            shuffled.push(x.clone());
            shuffled.push(nums[i as usize + n as usize].clone());
        }
       shuffled
    }
}