// Problem: String Matching in an Array
// Tags: String, String, String Matching
use std::collections::HashSet;
pub fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut matches: HashSet<String> = HashSet::new();

    for outer_word in &words {
        for inner_word in &words {
            if outer_word != inner_word {
                if outer_word.contains(inner_word) {
                    matches.insert(inner_word.clone());
                }
            }
        }
    }

    matches.into_iter().collect()
}
