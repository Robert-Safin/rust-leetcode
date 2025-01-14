// Problem: Construct K Palindrome Strings
// Tags: Hash Table, String, Greedy, Counting

pub fn can_construct(s: String, k: i32) -> bool {
    if s.len() < k as usize {
        return false;
    }

    let char_count: Vec<i32> = s.chars().fold(vec![0; 26], |mut acc, c| {
        acc[(c as u8 - b'a') as usize] += 1;
        acc
    });

    let odd_char_count = char_count.iter().fold(0, |mut acc, v| {
        if v % 2 != 0 {
            acc += 1;
        }
        acc
    });

    if odd_char_count > k {
        return false;
    }

    true
}
