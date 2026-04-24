impl Solution {
    fn sum_digits(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            let digit = n % 10;
            res += digit * digit;
            n /= 10;
        }
        res
    }
    pub fn is_happy(n: i32) -> bool {
        let mut hs = HashSet::<i32>::new();
        let mut sum = n;
        loop {
            sum = Self::sum_digits(sum);
            if sum == 1 {
                return true;
            } else if hs.contains(&sum) {
                return false;
            } else {
                hs.insert(sum);
            }
        }
    }
}
