// Problem: Count Prefix and Suffix Pairs I
// Tags: Array, String, Trie, Rolling Hash, String Matching, Hash Function
pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
    let mut count = 0;
    for outer in 0..words.len() {
        for inner in outer + 1..words.len() {
            if words[inner].starts_with(&words[outer]) && words[inner].ends_with(&words[outer]) {
                count += 1;
            }
        }
    }
    count
}
