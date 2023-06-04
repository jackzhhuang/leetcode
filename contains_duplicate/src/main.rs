
struct Solution;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        nums.into_iter().any(|item| {
            match set.get(&item) {
                Some(_) => true,
                None => {
                    set.insert(item);
                    false
                }
            }
        })
    }
}

fn main() {
   let result = Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]); 
   println!("{result}");
}