// Problem: Minimum Length of String After Operations
// Tags: Hash Table, String, Counting
//
pub struct Solution {}

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut counts: Vec<i32> = s.chars().fold(vec![0; 26], |mut acc, c| {
            acc[(c as u8 - b'a') as usize] += 1;
            acc
        });
        counts.iter_mut().for_each(|count| {
            while *count >= 3 {
                *count -= 2;
            }
        });
        counts.iter().sum()
    }
}
