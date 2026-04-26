impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0usize;
        let mut r = numbers.len() - 1;
        while l <= r {
            match (numbers[l] + numbers[r]).cmp(&target) {
                std::cmp::Ordering::Greater => r -= 1,
                std::cmp::Ordering::Equal => return vec![l as i32 + 1, r as i32 + 1],
                std::cmp::Ordering::Less => l += 1,
            }
        }
        unreachable!()
    }
}
