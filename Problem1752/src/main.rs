impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut ascending = 0;
        let n = nums.len();
        for i in 0..n {
            if nums[i] > nums[(i + 1) % n] {
                ascending += 1;
            }
        }
        ascending <= 1
    }
}
