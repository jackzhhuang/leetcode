
struct Solution {

}

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let v = sentence.chars().collect::<Vec<char>>();
        for w in v.windows(3).into_iter() {
            if w[1] != ' ' {
                continue;
            }
            if w[0] != w[2] {
                return false;
            }
        }
        return v[0] == v[v.len() - 1];
    }
}

fn main() {
    let result = Solution::is_circular_sentence("leetcode exercises sound delightful".to_string());
    if result {
        println!("is_circular_sentence returned true");
    } else {
        println!("is_circular_sentence returned false");
    }
}
