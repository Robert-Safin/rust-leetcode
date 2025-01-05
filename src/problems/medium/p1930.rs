// Problem: Unique Length-3 Palindromic Subsequences
// Tags: Hash Table, String, Bit Manipulation, Prefix Sum
use std::collections::{HashMap, HashSet};

pub fn count_palindromic_subsequence(s: String) -> i32 {
    let mut occurrences: HashMap<char, (isize, isize)> = HashMap::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, char) in chars.iter().enumerate() {
        if let Some(v) = occurrences.get_mut(&char) {
            if i as isize > v.1 {
                v.1 = i as isize;
            }
        } else {
            occurrences.insert(*char, (i as isize, i as isize));
        }
    }

    let mut count = 0;
    for (_, v) in occurrences {
        let diff = (v.1 - v.0) as i32;
        if diff > 1 {
            let slice = &chars[(v.0 + 1) as usize..=(v.1 - 1) as usize]
                .iter()
                .collect::<HashSet<&char>>();
            count += slice.len() as i32;
        }
    }

    return count as i32;
}
