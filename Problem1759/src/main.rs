impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let chars: Vec<char> = s.chars().collect();
        let mut count: i64 = 0;
        let mut streak: i64 = 1;

        for i in 1..chars.len() {
            if chars[i] == chars[i - 1] {
                streak += 1;
            } else {
                count = (count + streak * (streak + 1) / 2) % MOD;
                streak = 1;
            }
        }

        // Add the last streak
        count = (count + streak * (streak + 1) / 2) % MOD;

        count as i32
    }
}
