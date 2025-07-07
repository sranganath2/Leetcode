pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut maximum = 0;
    let mut sum = 0;
    for i in 0..gain.len() {
        sum += gain[i];
        if sum > maximum {
            maximum = sum;
        }
    }
    maximum
}
