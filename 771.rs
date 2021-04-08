/*
You're given strings jewels representing the types of stones that are jewels, and stones representing the stones you have. Each character in stones is a type of stone you have. You want to know how many of the stones you have are also jewels.

Letters are case sensitive, so "a" is considered a different type of stone from "A".

Runtime: 0 ms, faster than 100.00% of Rust online submissions for Jewels and Stones.
Memory Usage: 2 MB, less than 91.49% of Rust online submissions for Jewels and Stones.
*/

impl Solution 
{
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 
    {
        let mut jewel_count: i32 = 0;
        for c in jewels.chars()
        {
            jewel_count += stones.matches(c).count() as i32;
        }
        
        jewel_count
    }
}