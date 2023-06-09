struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut len = nums.len();
        let mut start = 0;
        let mut last = len - 1;
        while start < len {
            if nums[start] == val {
                while nums[last] == val && last != start {
                    len -= 1;
                    last -= 1;
                }
                if last == start {
                    return (len - 1) as i32;
                }
                let temp = nums[start];
                nums[start] = nums[last];
                nums[last] = temp;
                last -= 1;

                len -= 1;
            }                 
            start += 1;
        }
        len as i32
    }
}

fn main() {
    let mut input = vec![3, 2, 2, 3];
    let result = Solution::remove_element(&mut input, 3);
    for item in 0..result as usize {
        print!("{} ,", input[item]);
    }
    println!("");

    let mut input = vec![3, 3, 3, 3];
    let result = Solution::remove_element(&mut input, 3);
    for item in 0..result as usize {
        print!("{} ,", input[item]);
    }
    println!("");

    let mut input = vec![2, 2, 2, 2];
    let result = Solution::remove_element(&mut input, 3);
    for item in 0..result as usize {
        print!("{} ,", input[item]);
    }
    println!("");

    let mut input = vec![];
    let result = Solution::remove_element(&mut input, 3);
    for item in 0..result as usize {
        print!("{} ,", input[item]);
    }
    println!("");

    let mut input = vec![2];
    let result = Solution::remove_element(&mut input, 3);
    for item in 0..result as usize {
        print!("{} ,", input[item]);
    }
    println!("");

    let mut input = vec![3];
    let result = Solution::remove_element(&mut input, 3);
    for item in 0..result as usize {
        print!("{} ,", input[item]);
    }
    println!("");

    let mut input = vec![0,1,2,2,3,0,4,2];
    let result = Solution::remove_element(&mut input, 2);
    for item in 0..result as usize {
        print!("{} ,", input[item]);
    }
    println!("");


}
