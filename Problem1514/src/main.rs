use std::collections::VecDeque;
use std::f64::INFINITY;

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        let n = n as usize;
        let mut graph: Vec<Vec<(usize, f64)>> = vec![vec![]; n];
        for (i, edge) in edges.iter().enumerate() {
            let p = succ_prob[i];
            let w = -p.ln();
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push((v, w));
            graph[v].push((u, w));
        }

        let mut dist: Vec<f64> = vec![INFINITY; n];
        dist[start_node as usize] = 0.0;
        let mut q: VecDeque<usize> = VecDeque::default();
        q.push_front(start_node as usize);
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            for edge in &graph[u] {
                let &(v, w) = edge;
                if dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                    q.push_back(v);
                }
            }
        }

        (-dist[end_node as usize]).exp()
    }
}
