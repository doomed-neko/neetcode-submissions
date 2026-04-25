impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in digits.iter_mut().rev() {
            if i < &mut 9 {
                *i += 1;
                return digits;
            }
            *i = 0;
        }
        let mut res = vec![1];
        res.append(&mut digits);
        res
    }
}
