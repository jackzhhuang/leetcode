struct Solution {

}

impl Solution {
    pub fn reformat(s: String) -> String {
        let (chars, nums):(String, String) = s.chars().partition(|x| {
            x.is_alphabetic()
        });
        let (sign, long, short) = if chars.len() > nums.len() {
            (chars.len() - nums.len(), chars, nums)
        } else {
            (nums.len() - chars.len(), nums, chars)
        };

        if sign > 1 {
            return String::new();
        }

        long.chars().zip(short.chars()).map(|x|{
            format!("{}{}", x.0, x.1)
        }).collect::<String>() + &long[short.len()..]
    }
}

fn main() {
    let input = "a0b1c2".to_string();
    print!("{} = ", input);
    println!("{}", Solution::reformat(input));

    let input = "leetcode".to_string();
    print!("{} = ", input);
    println!("{}", Solution::reformat(input));

    let input = "1229857369".to_string();
    print!("{} = ", input);
    println!("{}", Solution::reformat(input));

    let input = "covid2019".to_string();
    print!("{} = ", input);
    println!("{}", Solution::reformat(input));

    let input = "ab123".to_string();
    print!("{} = ", input);
    println!("{}", Solution::reformat(input));
}
