impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut p = 1.;
        for i in 0..n{
            p *= x;
        }
        if n<0{
            for i in n..0{
                p /= x
            }
        }
        p
    }
}
