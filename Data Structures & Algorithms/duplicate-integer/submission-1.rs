impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut hs = HashMap::<i32, ()>::new();
        for i in nums {
            if let std::collections::hash_map::Entry::Vacant(e) = hs.entry(i) {
                e.insert(());
            } else {
                return true;
            }
        }
        false
    }
}
