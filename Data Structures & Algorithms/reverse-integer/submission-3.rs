impl Solution {
    pub fn reverse(y: i32) -> i32 {
        let r = (y as i64)
            .abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i64>()
            .unwrap()
            * y.signum() as i64;
        if r < i32::MIN as i64 || r > i32::MAX as i64 {
            0
        } else {
            r as i32
        }
    }
}
