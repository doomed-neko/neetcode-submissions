impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        fn my_search(l: i32, r: i32, nums: &[i32], target: i32) -> i32 {
            if l > r {
                return -1;
            }
            let mid = l + (r - l) / 2;
            match nums[mid as usize].cmp(&target) {
                std::cmp::Ordering::Less => my_search(mid + 1, r, nums, target),
                std::cmp::Ordering::Equal => mid,
                std::cmp::Ordering::Greater => my_search(l, mid - 1, nums, target),
            }
        }
        my_search(0, nums.len() as i32 - 1, &nums, target)
    }
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for i in matrix {
            if Self::search(i, target) >= 0 {
                return true;
            }
        }
        false
    }
}
