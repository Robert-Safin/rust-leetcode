// Problem: Defuse the Bomb
// Tags: Array, Sliding Window

pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let len = code.len();
    if k == 0 {
        return vec![0; len];
    }

    let mut out = vec![0; len];
    for i in 0..len {
        let sum: i32 = if k > 0 {
            (1..=k).map(|j| code[(i + j as usize) % len]).sum()
        } else {
            (1..=(-k)).map(|j| code[(i + len - j as usize) % len]).sum()
        };
        out[i] = sum;
    }

    out
}
