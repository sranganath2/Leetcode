impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let len = encoded.len();
        let mut res = Vec::with_capacity(len + 1);
        let xall = (1..=(len + 1)).reduce(|acc, e| acc ^ e).unwrap() as i32;
        let xen = encoded
            .iter()
            .skip(1)
            .step_by(2)
            .fold(0, |acc, el| acc ^ (*el));
        let first = xall ^ xen;
        res.push(first);
        encoded
            .into_iter()
            .for_each(|x| res.push(x ^ res.last().unwrap()));
        res
    }
}
