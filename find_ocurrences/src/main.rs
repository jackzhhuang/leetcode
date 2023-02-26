struct Solution {

}

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let vector_string: Vec<&str> = text.split_whitespace().collect();
        vector_string.windows(3).filter_map(|x| {
            if x[0].eq(&first) && x[1].eq(&second) {
                Some(x[2].to_string())
            } else {
                None
            }
        }).collect()
    }
}

fn main() {
    println!("the answer is {:?}", Solution::find_ocurrences("alice is a good girl she is a good student".to_string(), "a".to_string(), "good".to_string()));
    println!("the answer is {:?}", Solution::find_ocurrences("we will we will rock you".to_string(), "we".to_string(), "will".to_string()));
}
