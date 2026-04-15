// This program takes a sentence as input and converts each word into Pig Latin.

// Pig Latin rules:
// 1. If a word starts with a vowel, add "yay" to the end of the word.
// 2. If a word starts with a consonant,
//   - Move the first letter to the end of the word and add "ay".

// Example:
// Input: "Hello World"
// Output: "elloHay orldWay"

// Note: This program assumes that the input is a single sentence and does not handle punctuation or capitalization beyond the first letter.
// For more information on Pig Latin, see: https://en.wikipedia.org/wiki/Pig_Latin.

use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a sentence: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let result: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|word| to_pig_latin(word))
        .collect();

    println!("{}", result.join(" "))
}

fn to_pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut chars = word.chars();
    let first_letter = chars.next().unwrap();

    if vowels.contains(&first_letter.to_ascii_lowercase()) {
        return format!("{}-yay", word);
    } else {
        let rest_of_word: String = chars.collect();
        return format!("{}-{}ay", rest_of_word, first_letter);
    }
}
