struct Solution {

}

impl Solution {
    fn distance_inner(dis: &[i32], src: usize, dest: usize) -> i32 {
        let mut sum = 0;
        let mut next = src;
        loop {
            if next == dest {
                return sum;
            }
            sum += dis[next % dis.len()];
            next = (next + 1) % dis.len();
        }
    }
    pub fn distance_between_bus_stops(distance: Vec<i32>, 
                                      start: i32, 
                                      destination: i32) -> i32 {
        let first = Solution::distance_inner(&distance, start as usize, destination as usize);
        let second = Solution::distance_inner(&distance, destination as usize, start as usize);
        first.min(second)
    }
}

fn main() {
    println!("the shorter distance is {}", Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3));
}
