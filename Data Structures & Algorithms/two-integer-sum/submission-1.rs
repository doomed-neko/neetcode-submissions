impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm = HashMap::<i32, usize>::new();
        for (index, i) in nums.iter().enumerate() {
            if let Some(other_idx) = hm.get(&(target - i)) {
                return vec![*other_idx as i32, index as i32];
            } else {
                hm.insert(*i, index);
            }
        }
        vec![]
    }
}
