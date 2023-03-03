struct Solution {

}

impl Solution {
    /*
    for 2 bits, there is a regular: 
    00 => 0, the initial value, bit_count(00) = 0
    01: 01 & (01 - 1) = 01 & 00 = 00 => 0, hence, 0 + 1 = 1
    10: 10 & (10 - 1) = 10 & 01 = 00 => 0, hence, 0 + 1 = 1
    11: 11 & (11 - 1) = 11 & 10 = 10 => 1, hence, 1 + 1 = 2
    for those more thant 2 bits, for example, 100 which equals to 11 + 1:
    100: 100 & (100 - 1) = 100 & 011 => 000, it will be the way like 00, 
    but it has one more bit in the big endianness, 
    hence the count of bits is bit_count(100) = bit_count(000) + 1 = bit_count(00) + 1 = 1.
    101: 101 & (101 -1) = 101 & 100 => 100, bit_count(101) = bit_count(100) + 1 = 1 + 1 = 2.
    110, 111... and so on
    this is a cirle! 
    for y = x & (x - 1), we will have: bit_count(x) = bit_count(y) + 1
     */
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0; (n + 1) as usize];

        for i in 1..(n + 1) {
            ans[i as usize] = ans[(i & (i - 1)) as usize] + 1;
        }

        ans
    }
}

fn main() {
    println!("{:?}", Solution::count_bits(5));
}
