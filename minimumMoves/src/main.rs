struct Solution;
impl Solution {
   pub fn minimum_moves(s: String) -> i32 {
        let vec_chars = s.chars().into_iter().collect::<Vec<char>>();
        let mut count = 0;
        let mut i = 0;
        while i < vec_chars.len() {
            if vec_chars[i] == 'X' {
                i += 3;
                count += 1;
            } else {
                i += 1;
            }
        }
        count
    }
}

fn main() {
    println!("{}", Solution::minimum_moves("XXX".to_string()));
    println!("{}", Solution::minimum_moves("XXOX".to_string()));
    println!("{}", Solution::minimum_moves("OOOO".to_string()));
    println!("{}", Solution::minimum_moves("OXOX".to_string()));
}
