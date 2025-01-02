// Problem: Maximum Score After Splitting a String
// Tags: String, Prefix Sum

use std::cmp::max;

pub fn max_score(s: String) -> i32 {
    let mut max_score = i32::MIN;
    for i in 1..s.len() - 1 {
        let split = s.split_at(i);
        let left = split.0.chars().filter(|c| *c == '0').count();
        let right = split.1.chars().filter(|c| *c == '1').count();

        let sum = left + right;

        max_score = max(max_score, sum as i32)
    }

    max_score
}
