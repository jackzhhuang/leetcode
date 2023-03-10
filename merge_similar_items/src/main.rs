use std::collections::BTreeMap;
struct Solution;
impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut hash = BTreeMap::<i32, i32>::new();
        let mut ans = vec![];
        ans.reserve(items1.len() + items2.len());

        items1.into_iter().for_each(|item| {
            let mut next = item.into_iter();
            let index = next.next().unwrap();
            let weight = next.next().unwrap();
            hash.insert(index, weight);
        });
        items2.into_iter().for_each(|item| {
            let mut next = item.into_iter();
            let index = next.next().unwrap();
            let weight = next.next().unwrap();
            hash.entry(index).and_modify(|x| *x += weight ).or_insert(weight);
        });

        hash.into_iter().for_each(|(key, value)| {
            ans.push(vec![key, value]);
        });
        ans
    }
}

fn main() {
    println!("{:?}", Solution::merge_similar_items(vec![vec![1,1],vec![4,5],vec![3,8]],  vec![vec![3,1],vec![1,5]]));
    println!("{:?}", Solution::merge_similar_items(vec![vec![1,1],vec![3,2],vec![2,3]],  vec![vec![2,1],vec![3,2],vec![1,3]]));
    println!("{:?}", Solution::merge_similar_items(vec![vec![1,3],vec![2,2]],  vec![vec![7,1],vec![2,2],vec![1,4]]));
}
