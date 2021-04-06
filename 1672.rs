/*
You are given an m x n integer grid accounts where accounts[i][j] is the amount of money the i​​​​​​​​​​​th​​​​ customer has in the j​​​​​​​​​​​th​​​​ bank. Return the wealth that the richest customer has.

A customer's wealth is the amount of money they have in all their bank accounts. The richest customer is the customer that has the maximum wealth.

Runtime: 0 ms, faster than 100.00% of Rust online submissions for Richest Customer Wealth.
Memory Usage: 2.3 MB, less than 11.02% of Rust online submissions for Richest Customer Wealth.
*/

impl Solution 
{
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 
    {
        let mut max: i32 = 0;
        
        for customer in accounts.iter()
        {
            let wealth: i32 = customer.iter().sum();
            
            if wealth > max 
            {
                max = wealth;
            }
        }
        
        max
    }
}