impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len()) .map(|i| { nums.iter() .enumerate() .filter(|(x, _)| x != &i) .map(|(_, x)| x) .product() }) .collect::<Vec<i32>>()
    }
}
