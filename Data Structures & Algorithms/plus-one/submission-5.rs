impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in digits.iter_mut().rev() {
            if *i < 9 {
                *i += 1;
                return digits;
            }
            *i = 0;
        }
        [vec![1], digits].into_iter().flatten().collect()
    }
}
