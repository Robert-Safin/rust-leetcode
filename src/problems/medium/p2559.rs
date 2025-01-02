// Problem: Count Vowel Strings in Ranges
// Tags: Array, Prefix Sum, String

pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut out: Vec<i32> = vec![];
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut pre_fix: Vec<i32> = vec![];
    let mut current = 0;

    for word in words {
        let chars: Vec<char> = word.chars().collect();
        if vowels.contains(chars.first().unwrap()) && vowels.contains(chars.last().unwrap()) {
            current += 1;
        }
        pre_fix.push(current);
    }

    for query in queries {
        let (start_i, end_i) = (query[0], query[1]);

        let end = pre_fix[end_i as usize];

        let start = if start_i == 0 {
            0
        } else {
            pre_fix[start_i as usize - 1]
        };

        out.push(end - start);
    }
    out
}
