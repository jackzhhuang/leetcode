struct Solution {

}
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut left_hash = vec![s.len(); 256];
        let mut right_hash = vec![t.len(); 256];

        for (i, (left, right)) in s.bytes().zip(t.bytes()).enumerate() {
            if left_hash[left as usize] != right_hash[right as usize] {
                return false;
            }
            left_hash[left as usize] = i;
            right_hash[right as usize] = i;
       }

        true
    }
}

fn test() {
    let result = Solution::is_isomorphic("egg".to_string(), "add".to_string());
    println!("is_isomorphic returned: {}", result);

    let result = Solution::is_isomorphic("foo".to_string(), "bar".to_string());
    println!("is_isomorphic returned: {}", result);

    let result = Solution::is_isomorphic("paper".to_string(), "title".to_string());
    println!("is_isomorphic returned: {}", result);

    let result = Solution::is_isomorphic("badc".to_string(), "baba".to_string());
    println!("is_isomorphic returned: {}", result);
}

fn main() {
   test(); 
}