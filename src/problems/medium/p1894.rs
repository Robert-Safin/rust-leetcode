// Problem: Find the Student that Will Replace the Chalk
// Tags: Array, Binary Search, Simulation, Prefix Sum

pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
    let chalk_sum: i64 = chalk.iter().map(|&x| x as i64).sum();

    let mut remain: i64 = (k as i64) % chalk_sum;

    for (i, &v) in chalk.iter().enumerate() {
        if remain < v as i64 {
            return i as i32;
        }
        remain -= v as i64;
    }

    unreachable!()
}
