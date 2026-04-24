impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut sum = 0i32;
        for i in 0..32 {
            sum += (((1 << i) & n) >> i) as i32;
        }
        sum
    }
}
