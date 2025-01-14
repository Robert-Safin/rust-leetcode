// Problem: Word Subsets
// Tags: Array, Hash Table, String
pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    let calculate_frequency = |word: &str| {
        let mut freq = vec![0; 26];
        word.chars()
            .for_each(|c| freq[(c as u8 - b'a') as usize] += 1);
        freq
    };

    let max_frequencies =
        words2
            .iter()
            .map(|word| calculate_frequency(word))
            .fold(vec![0; 26], |mut acc, freq| {
                acc.iter_mut()
                    .zip(freq.iter())
                    .for_each(|(a, &f)| *a = (*a).max(f));
                acc
            });

    words1
        .into_iter()
        .filter(|word| {
            let freq = calculate_frequency(word);
            max_frequencies
                .iter()
                .zip(freq.iter())
                .all(|(&max_f, &f)| f >= max_f)
        })
        .collect()
}
