impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        let mut s = vec![0; l];
        let mut p = vec![0; l];
        let mut r = vec![0; l];
        p[0] = 1;
        s[l - 1] = 1;

        for i in 1..l {
            p[i] = p[i - 1] * nums[i - 1];
        }
        for i in (0..l - 1).rev() {
            s[i] = s[i + 1] * nums[i + 1];
        }
        for i in 0..l {
            r[i] = p[i] * s[i];
        }

        r
    }
}
