struct Solution;
impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let b = s.bytes().into_iter().collect::<Vec<u8>>();
        let mut dist = distance.clone(); 
        for (index, x) in b.iter().enumerate() {
            let pos = *x as usize - 'a' as usize;
            if dist[pos] == -1 {
                continue;
            }
            let next = index + dist[pos] as usize + 1;
            if next < b.len() {
                if b[next] != *x {
                    return false;
                } else {
                    dist[pos] = -1;
                }
            } else {
                return false;
            }
        }
        true
    }
}
fn main() {
    println!("{}", Solution::check_distances("abaccb".to_string(), vec![1,3,0,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]));
    println!("{}", Solution::check_distances("aa".to_string(), vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]));
}
