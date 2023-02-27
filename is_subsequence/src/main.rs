struct Solution {

}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iter = t.chars();
        s.chars().filter(|x| {
            while let Some(y) = iter.next() {
                if x == &y {
                    return true;
                }
            }
            false
        }).collect::<String>().len() == s.len()
    }
}

fn main() {
    println!("abc is ahbgdc subsequence: {}", Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()));
    println!("axc is ahbgdc subsequence: {}", Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()));
}
