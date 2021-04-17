/*
There is a hidden integer array arr that consists of n non-negative integers.

It was encoded into another integer array encoded of length n - 1, such that encoded[i] = arr[i] XOR arr[i + 1]. For example, if arr = [1,0,2,1], then encoded = [1,2,3].

You are given the encoded array. You are also given an integer first, that is the first element of arr, i.e. arr[0].

Return the original array arr. It can be proved that the answer exists and is unique.

Runtime: 8 ms, faster than 96.72% of Rust online submissions for Decode XORed Array.
Memory Usage: 2.2 MB, less than 32.79% of Rust online submissions for Decode XORed Array.
*/

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> 
    {
        let mut arr = vec![first];
        
        for (i, x) in encoded.iter().enumerate()
        {
            arr.push(arr[i] ^ x);
        }
        
        arr
    }
}