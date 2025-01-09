// Problem: Counting Words With a Given Prefix
// Tags: Array, String, String Matching

pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    words
        .into_iter()
        .filter(|word| word.starts_with(&pref))
        .count() as i32
}
