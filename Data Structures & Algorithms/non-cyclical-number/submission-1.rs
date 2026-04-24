impl Solution {
    fn sum_digits(n: i32) -> i32 {
        n.to_string()
            .chars()
            .map(|x| x.to_string().parse::<i32>().unwrap().pow(2))
            .sum()
    }
    pub fn is_happy(n: i32) -> bool {
        let mut hs = HashSet::<i32>::new();
        let mut sum = Self::sum_digits(n);
        loop {
            if sum == 1 {
                return true;
            } else if hs.contains(&sum) {
                return false;
            } else {
                hs.insert(sum);
                sum = Self::sum_digits(sum);
            }
        }
    }
}
