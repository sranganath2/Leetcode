impl Solution {
    pub fn maximum_time(t: String) -> String {
        let mut r = String::from("");
        for (k, c) in t.chars().enumerate() {
            if c != '?' {
                r.push(c);
                continue;
            }
            r.push(match k {
                0 => {
                    if t.chars().nth(1) < Some('4') || t.chars().nth(1) == Some('?') {
                        '2'
                    } else {
                        '1'
                    }
                }
                1 => {
                    if r.chars().nth(0) == Some('2') {
                        '3'
                    } else {
                        '9'
                    }
                }
                3 => '5',
                4 => '9',
                _ => unreachable!(),
            })
        }
        r
    }
}
