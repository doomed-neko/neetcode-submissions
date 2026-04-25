use std::ops::Add;
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        digits
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<i64>()
            .unwrap()
            .add(1)
            .to_string()
            .chars()
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    }
}
