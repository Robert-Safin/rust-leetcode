// Problem: Longest Substring Without Repeating Characters
// Tags: Hash Table, String,  Sliding Window
use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars = s.chars().collect::<Vec<char>>();
    let mut set: HashSet<char> = HashSet::new();
    let mut max_length = 0;
    let mut left = 0;

    for right in 0..chars.len() {
        while set.contains(&chars[right]) {
            set.remove(&chars[left]);
            left += 1;
        }

        set.insert(chars[right]);
        max_length = max_length.max(right - left + 1);
    }

    max_length as i32
}
