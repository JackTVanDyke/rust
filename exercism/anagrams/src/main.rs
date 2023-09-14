// Given "listen" and a list of candidates like "enlists" "google" "inlets" "banana" the program should return a list containing "inlets".

// The solution is case insensitive, which means "WOrd" is the same as "word" or "woRd".

use std::{collections::HashSet};

fn sort_word(word: &str) -> Vec<char> {
    let mut sort_word: Vec<char> = word.chars().collect();
    sort_word.sort_unstable();
    sort_word
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut res: HashSet<&str> = HashSet::new();
    let word = word.to_lowercase();
    let word_sort = sort_word(&word);
    for x in possible_anagrams.iter() {
        let ana_word = x.to_lowercase();
        if word.len() == ana_word.len()
        && word != ana_word && sort_word(&ana_word) == word_sort {
            res.insert(x);
        }
    }
    res
}

fn main() {
    println!("{:?}", anagrams_for("Orchestra", &["cashregister", "Carthorse", "radishes"]))
}
