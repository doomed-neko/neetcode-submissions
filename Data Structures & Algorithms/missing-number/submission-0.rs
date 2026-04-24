impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut x = 0;
        nums.iter().for_each(|i| x ^= i);
        (0..nums.len() as i32 + 1).for_each(|i| x ^= i);
        x
    }
}
