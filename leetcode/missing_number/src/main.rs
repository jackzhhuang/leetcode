struct Solution {
}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let max = nums.iter().fold(0, |acc, x| {
            sum += *x;
            if acc > *x {
                acc
            } else {
                *x
            }
        });

        if max == nums.len() as i32 - 1 {
            return nums.len() as i32;
        }

        (max + 1) * max / 2 - sum
    }
}

fn test(nums: Vec<i32>) {
    print!("the missing nums in {:?} ", nums);
    println!("is {}", Solution::missing_number(nums));
}

fn main() {
    test(vec![3, 0, 1]);
    test(vec![0, 1]);
    test(vec![9,6,4,2,3,5,7,0,1]);
    test(vec![0]);
}
