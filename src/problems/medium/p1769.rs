// Problem: Minimum Number of Operations to Move All Balls to Each Box
// Tags: String, Array, Prefix Sum

pub fn min_operations(boxes: String) -> Vec<i32> {
    let mut left: (i32, i32) = (0, 0);
    let mut out_left: Vec<i32> = vec![0; boxes.len()];
    for (i, n) in boxes.chars().enumerate() {
        out_left[i] += left.0;
        left.1 += n.to_digit(10).unwrap() as i32;
        left.0 += left.1;
    }

    let mut right: (i32, i32) = (0, 0);
    let mut out_right: Vec<i32> = vec![0; boxes.len()];
    for (i, n) in boxes.chars().rev().enumerate() {
        out_right[i] += right.0;
        right.1 += n.to_digit(10).unwrap() as i32;
        right.0 += right.1;
    }

    out_right.reverse();

    out_left
        .iter()
        .zip(out_right)
        .map(|(a, b)| a + b)
        .collect::<Vec<i32>>()
}
