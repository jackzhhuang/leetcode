struct Solution;
impl Solution {
    fn co(mut num1: i32, mut num2: i32) -> i32 {
        let mut count = 0;
        loop {
            if num1 == 0 || num2 == 0 {
                return count;
            }
            if num1 >= num2 {
                num1 = num1 - num2;
                count += 1;
            } else {
                num2 = num2 - num1;
                count += 1;
            }
        }
    }
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        Solution::co(num1, num2)
    }
}

fn main() {
    println!("{}", Solution::count_operations(5, 4));
    println!("{}", Solution::count_operations(2, 3));
    println!("{}", Solution::count_operations(10, 10));
    println!("{}", Solution::count_operations(0, 0));
}
