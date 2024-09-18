use std::collections::HashMap;

pub fn uncommon_words(s1: &str, s2: &str) -> Vec<String> {
    let mut word_count = HashMap::new();

    for word in s1.split_whitespace().chain(s2.split_whitespace()) {
        *word_count.entry(word).or_insert(0) += 1;
    }

    word_count
        .into_iter()
        .filter(|&(_, count)| count == 1)
        .map(|(word, _)| word.to_string())
        .collect()
}
