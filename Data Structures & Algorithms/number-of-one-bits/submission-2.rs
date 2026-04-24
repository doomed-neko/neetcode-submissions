impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut n = n;
        let mut sum = 0;
        while n > 0 {
            sum += n & 1;
            n >>= 1;
        }
        sum as i32
    }
}
