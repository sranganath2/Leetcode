use std::collections::HashMap;
impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        fn digit_sum(mut n: i32) -> i32 {
            let mut sum = 0;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }
            sum
        }

        let mut box_counts: HashMap<i32, i32> = HashMap::new();
        let mut max_balls = 0;

        for number in low_limit..=high_limit {
            let box_number = digit_sum(number);
            let count = box_counts.entry(box_number).or_insert(0);
            *count += 1;
            if *count > max_balls {
                max_balls = *count;
            }
        }

        max_balls
    }
}
