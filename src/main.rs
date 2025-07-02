pub fn main(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (n, m, modulo) = (grid.len(), grid[0].len(), 12345u64);
    let (mut prefix, mut suffix, mut x) = (vec![1; n * m + 1], vec![1; n * m + 1], 1);
    for i in 0..n {
        for j in 0..m {
            prefix[x] = (prefix[x - 1] * grid[i][j] as u64) % modulo;
            x += 1;
        }
    }
    let mut x = n * m;
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            x -= 1;
            suffix[x] = (suffix[x + 1] * grid[i][j] as u64) % modulo;
        }
    }
    let mut x = 0;
    for i in 0..n {
        for j in 0..m {
            grid[i][j] = ((prefix[x] * suffix[x + 1]) % modulo) as i32;
            x += 1;
        }
    }
    grid
}
