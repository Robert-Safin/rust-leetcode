// Problem: Valid Anagram
// Tags: String, Hash Table, Sorting

use std::collections::HashMap;
pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_map: HashMap<char, i32> = HashMap::new();
    let mut t_map: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        if let Some(v) = s_map.get_mut(&c) {
            *v += 1;
        } else {
            s_map.insert(c, 1);
        }
    }

    for c in t.chars() {
        if let Some(v) = t_map.get_mut(&c) {
            *v += 1;
        } else {
            t_map.insert(c, 1);
        }
    }

    if s_map.keys().count() != t_map.keys().count() {
        return false;
    }

    for (k, v) in s_map {
        if let Some(other) = t_map.get(&k) {
            if v != *other {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}
