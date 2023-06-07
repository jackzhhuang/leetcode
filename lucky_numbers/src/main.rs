use std::cmp::min;

struct Solution;
impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut row_result = Vec::new();
        let mut col_result = Vec::new();
        let mut col = Vec::new();
        matrix.iter().enumerate().for_each(|(index, row)| {
            if row.is_empty() {
                return;
            }
            row_result.push(*row.iter().min().unwrap());
            if index == 0 {
                row.iter().for_each(|item| {
                    col.push(vec![item]);
                });
            } else {
                 row.iter().enumerate().for_each(|(s, item)| {
                    col[s].push(item);
                });
            }
        });
        if col.is_empty() {
            return [].to_vec();
        }
        col.into_iter().for_each(|items| {
            col_result.push(**items.iter().max().unwrap());
        });
        row_result.into_iter().filter(|item| col_result.contains(item)).collect()
    }
}

fn main() {
    let result = Solution::lucky_numbers([[3,7,8].to_vec(),[9,11,13].to_vec(),[15,16,17].to_vec()].to_vec());
    println!("{result:?}");
    let result = Solution::lucky_numbers([[].to_vec(),[].to_vec(),[].to_vec()].to_vec());
    println!("{result:?}");
    let result = Solution::lucky_numbers([[].to_vec()].to_vec());
    println!("{result:?}");
    let result = Solution::lucky_numbers([[1,10,4,2].to_vec(),[9,3,8,7].to_vec(),[15,16,17,12].to_vec()].to_vec());
    println!("{result:?}");
    let result = Solution::lucky_numbers([[7, 8].to_vec(),[1, 2].to_vec()].to_vec());
    println!("{result:?}");



}
