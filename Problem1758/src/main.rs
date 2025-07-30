impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut pattern1 = 0; // starting with '0'
        let mut pattern2 = 0; // starting with '1'

        for (i, &ch) in bytes.iter().enumerate() {
            if i % 2 == 0 {
                if ch != b'0' {
                    pattern1 += 1;
                }
                if ch != b'1' {
                    pattern2 += 1;
                }
            } else {
                if ch != b'1' {
                    pattern1 += 1;
                }
                if ch != b'0' {
                    pattern2 += 1;
                }
            }
        }
        pattern1.min(pattern2)
    }
}
