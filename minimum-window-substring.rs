use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn contain_substring(map: &HashMap<char, i32>) -> bool {
        for cnt in map.values() {
            if *cnt > 0 {
                return false;
            }
        }
        true
    }

    pub fn min_window(s: String, t: String) -> String {
        let mut res = String::from("");
        let mut substring = String::from("");
        let mut map = HashMap::new();

        for c in t.chars() {
            let cnt = map.entry(c).or_insert(0);
            *cnt += 1;
        }
        
        for (j, c) in s.chars().enumerate() {
            substring.push(c);
            if let Some(cnt) = map.get_mut(&c) {
                *cnt -= 1;
            }
             
            while Solution::contain_substring(&map) {
                if (res.len() == 0) || (res.len() > substring.len()) {
                    res = substring.clone();
                }
                let c = substring.remove(0);
                if let Some(cnt) = map.get_mut(&c) {
                    *cnt += 1;
                }
            }
        }

        res
    }
}

fn main() {
    let ret = Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABC"));
    println!("{ret}");
}