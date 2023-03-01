struct Solution {

} 

impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let mut right_sum = nums.iter().sum::<i32>() - nums[0];
        let mut left_sum = 0;
        let mut index = 0;
        loop {
            if right_sum == left_sum {
                return index as i32;
            }
            index += 1;
            if index >= nums.len() {
                break;
            }
            left_sum += nums[index - 1];
            right_sum -= nums[index];
        }
        return -1;
    }
}
fn main() {
    let input = vec![2, 3, -1, 8, 4];
    println!("mid index is {}", Solution::find_middle_index(input));

    let input = vec![1, -1, 4];
    println!("mid index is {}", Solution::find_middle_index(input));

    let input = vec![2, 5];
    println!("mid index is {}", Solution::find_middle_index(input));

    let input = vec![1];
    println!("mid index is {}", Solution::find_middle_index(input));


}
