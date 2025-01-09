// Problem: Permutation in String
// Tags: Hash Table, Two Pointers, String, Sliding Window

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }
    let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
    let (mut s1_count, mut s2_count) = ([0; 26], [0; 26]);

    for i in 0..s1.len() {
        s1_count[(s1[i] - b'a') as usize] += 1;
        s2_count[(s2[i] - b'a') as usize] += 1;
    }

    if s1_count == s2_count {
        return true;
    }

    for i in s1.len()..s2.len() {
        s2_count[(s2[i] - b'a') as usize] += 1;
        s2_count[(s2[i - s1.len()] - b'a') as usize] -= 1;

        if s1_count == s2_count {
            return true;
        }
    }

    false
}
