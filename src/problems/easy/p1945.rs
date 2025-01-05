// Problem: Sum of Digits of String After Convert
// Tags: String, Simulation

pub fn get_lucky(s: String, k: i32) -> i32 {
    let mut sum_str = s
        .chars()
        .map(|c| (c as u8) - b'a' + 1)
        .map(|n| n.to_string())
        .collect::<String>();

    for _ in 0..k {
        sum_str = sum_str
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum::<u32>()
            .to_string();
    }

    sum_str.parse::<i32>().unwrap()
}
