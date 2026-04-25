impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut p = 1.;
        if x == 0.{return 0.}
        if n==0 {return 1.}
        for _ in 0..(n as i64).abs(){
            p *= x;
        }
        if n>=0{p} else{1./p}
    }
}
