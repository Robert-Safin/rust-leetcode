// Problem: Group Anagrams
// Tags: Array, String, Hash Table, Sorting
use std::collections::HashMap;
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for s in &strs {
        let mut sorted_str: Vec<char> = s.chars().collect();
        sorted_str.sort();
        let sorted_str: String = sorted_str.iter().collect();

        if let Some(v) = map.get_mut(&sorted_str) {
            v.push(s.to_owned());
        } else {
            map.insert(sorted_str, vec![s.to_owned()]);
        }
    }
    let mut out = vec![];

    for (_, v) in map {
        out.push(v);
    }

    out
}
