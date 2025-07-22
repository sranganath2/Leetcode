impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut min_sum = nums[0];
        let mut current_max = 0;
        let mut current_min = 0;

        for num in nums {
            current_max = current_max.max(0) + num;
            current_min = current_min.min(0) + num;

            max_sum = max_sum.max(current_max);
            min_sum = min_sum.min(current_min);
        }

        max_sum.max(min_sum.abs())
    }
}
