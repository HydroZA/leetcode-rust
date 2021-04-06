/*
Given a valid (IPv4) IP address, return a defanged version of that IP address.
A defanged IP address replaces every period "." with "[.]".

@Author James Legge
Runtime: 0ms, 2.2MB Memory, faster than 100% of Rust solutions
*/

impl Solution {
    pub fn defang_i_paddr(address: String) -> String 
    {
        let mut defang_ip = address.clone();
        str::replace(&defang_ip[..], ".", "[.]")
    }
}