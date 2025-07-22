use std::collections::HashMap;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj = HashMap::new();
        for p in adjacent_pairs {
            adj.entry(p[0]).or_insert(Vec::new()).push(p[1]);
            adj.entry(p[1]).or_insert(Vec::new()).push(p[0]);
        }
        let mut prev = i32::MIN;
        let mut cur = *adj
            .iter()
            .find(|(k, v)| v.len() == 1)
            .map(|(k, _)| k)
            .unwrap();

        let mut ans = Vec::new();
        loop {
            ans.push(cur);
            if let Some(&next) = adj[&cur].iter().filter(|&&n| n != prev).next() {
                prev = cur;
                cur = next;
            } else {
                break;
            }
        }
        ans
    }
}
