use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    //todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let chars = word.to_lowercase()
        .chars()
        //.map(|c| c.to_lowercase())
        .collect::<Vec<char>>();
    let mut anagrams = HashSet::new();
    for possible_anagram in possible_anagrams {
        let possible_chars = possible_anagram.to_lowercase().chars().collect::<Vec<char>>();
        if possible_chars.len() == chars.len()
            && !possible_anagram.to_lowercase().contains(&word.to_lowercase())
            && check_char_cardinality(word, possible_anagram)
            && possible_chars
                .iter()
                .all(|c| chars.contains(c))
        {
            anagrams.insert(*possible_anagram);
        }
    }
    anagrams
}

fn check_char_cardinality(word: &str, possible_anagram: &str) -> bool {
    let mut a: HashMap<char, i32> = HashMap::new();
    word.to_lowercase().chars().for_each(|c| {
        let count = a.entry(c).or_insert(0);
        *count += 1;
    });
    let mut b: HashMap<char, i32> = HashMap::new();
    possible_anagram.to_lowercase().chars().for_each(|c| {
        let count = b.entry(c).or_insert(0);
        *count += 1;
    });
    a.keys().all(|k| a[k] == *b.get(k).unwrap_or(&0))
}
