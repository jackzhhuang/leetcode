struct Solution {

}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_nums = std::collections::HashMap::<i32, i32>::new();
        for (index, value) in nums.iter().enumerate() {
            let result = hash_nums.get(&(target - value));
            if let Some(other_index) = result {
                return vec![index as i32, *other_index as i32];
            } else {
                hash_nums.insert(*value, index as i32);
            }
        }

        return vec![];
    }
}

fn main() {
    let result = Solution::two_sum(vec![3, 2, 4], 6);
    println!("the answer is {:?}", result);

    let result = Solution::two_sum(vec![2,7,11,15], 9);
    println!("the answer is {:?}", result);

    let result = Solution::two_sum(vec![3, 3], 6);
    println!("the answer is {:?}", result);
}
