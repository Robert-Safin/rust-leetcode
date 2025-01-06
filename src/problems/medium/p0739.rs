// Problem: Daily Temperatures
// Tags: Stack, Array, Monotonic Stack
//
//
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::new();
    let mut result = vec![0; temperatures.len()];

    for (i, &temp) in temperatures.iter().enumerate() {
        while let Some(&(prev_temp, prev_i)) = stack.last() {
            if temp > prev_temp {
                result[prev_i] = (i - prev_i) as i32;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push((temp, i));
    }
    result
}
