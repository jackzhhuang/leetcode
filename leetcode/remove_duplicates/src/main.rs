struct Solution {

}

impl Solution {
    fn find_next_diff(value: i32, begin: usize, nums: &[i32]) -> usize {
        for index in begin..nums.len() {
            if value != nums[index] {
                return index;
            }
        }
        nums.len()
    }
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let mut current: usize = 0;
        let mut begin: usize = 1;
        let mut count = 1;
        loop {
            let next = Solution::find_next_diff(nums[current], begin, nums);
            if next == nums.len() {
                break;
            }
            if next == current + 1 {
                current = next;
            } else {
                current += 1;
                nums[current] = nums[next];
            }             
            count += 1;
            begin = Solution::find_next_diff(nums[next], next + 1, nums);
        }
        count
    }
}

fn test(v: &mut Vec<i32>) {
    let len = Solution::remove_duplicates(v);
    println!("len = {}, v = {:?}", len, v);
}

fn main() {
    test(&mut vec![1, 1, 2]);
    test(&mut vec![0,0,1,1,1,2,2,3,3,4]);
}
