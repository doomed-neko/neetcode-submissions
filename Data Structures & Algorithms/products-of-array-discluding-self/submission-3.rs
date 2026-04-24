impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len())
            .map(|i| {
                nums.iter()
                    .enumerate()
                    .filter_map(|(x, j)| if x != i { Some(j) } else { None })
                    .product()
            })
            .collect::<Vec<i32>>()
    }
}
