impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut p = 1.;
        for i in 0..n.abs(){
            p *= x;
        }
        if n>=0{p} else{1./p}
    }
}
