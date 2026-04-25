impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn p(x:f64, n:i64)->f64{
            if x == 0.{ return 0.}
            if n == 0 { return 1.}
            let h = p(x,n/2);
            if n%2==0 { h*h } else { x*h*h}
        }
        let r = p(x, (n as i64).abs());
        if n >=0 {r} else {1./r}
    }
}
