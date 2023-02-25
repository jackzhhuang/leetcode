struct Solution {

}
impl Solution {
    fn inner(s: &str, t: &String) -> bool {
        let mut hash = std::collections::HashMap::new();

        let lefts = s.chars().collect::<Vec<char>>();
        let rights = t.chars().collect::<Vec<char>>();

        for index in 0..lefts.len() {
            let r_op = hash.get(&lefts[index]);
            match r_op {
                None => {
                    hash.insert(lefts[index], rights[index]);
                }
                Some(r) => {
                    if *r != rights[index] {
                        return false;
                    }
                }
            }
        }

        true
    }
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if !Solution::inner(&s, &t) {
            return false;
        }
        if !Solution::inner(&t, &s) {
            return false;
        }
        true
    }
}

fn main() {
    let result = Solution::is_isomorphic("egg".to_string(), "add".to_string());
    println!("is_isomorphic returned: {}", result);

    let result = Solution::is_isomorphic("foo".to_string(), "bar".to_string());
    println!("is_isomorphic returned: {}", result);

    let result = Solution::is_isomorphic("paper".to_string(), "title".to_string());
    println!("is_isomorphic returned: {}", result);

    let result = Solution::is_isomorphic("badc".to_string(), "baba".to_string());
    println!("is_isomorphic returned: {}", result);
}
