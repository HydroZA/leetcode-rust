/*
Given the array candies and the integer extraCandies, where candies[i] represents the number of candies that the ith kid has.

For each kid check if there is a way to distribute extraCandies among the kids such that he or she can have the greatest number of candies among them. Notice that multiple kids can have the greatest number of candies.

Runtime: 0 ms, faster than 100.00% of Rust online submissions for Kids With the Greatest Number of Candies.
Memory Usage: 2.1 MB, less than 26.32% of Rust online submissions for Kids With the Greatest Number of Candies.
*/
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        // find out what the max number is
        // any candies[i] + extra >= than max = true else false
        let max = match candies.iter().max()
        {
            Some(n) => n,
            None => &0i32
        };
        
        let mut out = vec![];
        
        for candy in candies.iter()
        {
            if candy + extra_candies >= *max
            {
                out.push(true);
            }
            else
            {
                out.push (false);
            }
        }
        
        out
    }
}