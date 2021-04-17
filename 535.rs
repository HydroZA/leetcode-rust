/*
TinyURL is a URL shortening service where you enter a URL such as https://leetcode.com/problems/design-tinyurl 
and it returns a short URL such as http://tinyurl.com/4e9iAk.

Design the encode and decode methods for the TinyURL service. 
There is no restriction on how your encode/decode algorithm should work. 
You just need to ensure that a URL can be encoded to a tiny URL and the tiny URL can be decoded to the original URL.

Runtime: 0 ms, faster than 100.00% of Rust online submissions for Encode and Decode TinyURL.
Memory Usage: 2.2 MB, less than 46.94% of Rust online submissions for Encode and Decode TinyURL.
*/

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct Codec
{
    urls: HashMap<String, String>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self 
    {
        Self
        {
            urls: HashMap::new()
        }
    }
	
    // Encodes a URL to a shortened URL.
    fn encode(&mut self, longURL: String) -> String 
    {
        let mut hasher = DefaultHasher::new();
        longURL.hash(&mut hasher);

        let shortURL: String = format!("http://tinyurl.com/{}", hasher.finish().to_string());
        self.urls.insert(shortURL.clone(), longURL);

        shortURL
    }
	
    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String 
    {
        match self.urls.get(&shortURL)
        {
            Some(longURL) => longURL.to_string(),
            None => String::new()
        }
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */

fn main()
{
    let mut codec = Codec::new();

    let long_url = String::from("https://doc.rust-lang.org/std/string/trait.ToString.html");
    let short_url = codec.encode(long_url);

    println!("{}", &short_url);
    println!("{}", codec.decode(short_url));
}
