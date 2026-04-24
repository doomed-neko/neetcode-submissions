impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut buckets: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];
        for i in nums {
            if let Some(v) = hm.get_mut(&i) {
                *v += 1;
            } else {
                hm.insert(i, 1);
            }
        }
        for (num, freq) in hm {
            buckets[freq].push(num);
        }
        buckets
            .into_iter()
            .rev()
            .flatten()
            .take(k as usize)
            .collect()
    }
}
