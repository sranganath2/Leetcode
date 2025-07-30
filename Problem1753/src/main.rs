impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let total = a + b + c;
        let max_val = a.max(b).max(c);
        let sum_of_other_two = total - max_val;
        sum_of_other_two.min(total / 2)
    }
}
