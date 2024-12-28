use rand::seq::SliceRandom;
use rand::Rng;
use clearscreen::clear;
use std::collections::HashSet;
use std::io;

/// Computes the score for a given word based on its length and unique characters.
/// 
/// # Parameters
/// - `word`: The word for which the score will be calculated.
/// 
/// # Returns
/// The score for the word. The score is based on the word's length, with a bonus for having 7 unique characters.
/// If the word is a pangram (7 unique letters), a bonus of 7 points is awarded.
///
pub fn compute_score(word: &str) -> usize {
    let mut score = 0;

    // Add bonus for 7 unique characters
    if word.chars().collect::<HashSet<_>>().len() == 7 {
        println!("Pangram!");
        score += 7;
    }

    // Score based on length
    score += if word.len() == 4 { 1 } else { word.len() };
    score
}

/// Loads the word list from a file and stores each word in a `HashSet` for efficient lookup.
/// 
/// # Parameters
/// - `file_path`: The path to the file containing the word list. Each word should be on its own line.
/// 
/// # Returns
/// A `HashSet<String>` containing all the words in the file.
///
pub fn load_word_list(file_content: &str) -> HashSet<String> {
    let mut word_set = HashSet::new();
    for line in file_content.lines() {
        word_set.insert(line.trim().to_string()); // Store each word
    }
    word_set
}

fn get_random_line(file_content: &str) -> Option<String> {
    let lines: Vec<&str> = file_content.lines().collect();
    let total_lines = lines.len();

    if total_lines == 0 {
        return None;
    }

    let random_index = rand::thread_rng().gen_range(0..total_lines);
    Some(lines[random_index].to_string())
}

/// Validates whether a given word exists in the loaded word list.
/// 
/// # Parameters
/// - `word`: The word to validate.
/// - `word_list`: A reference to the `HashSet<String>` containing valid words.
/// 
/// # Returns
/// `true` if the word exists in the word list, otherwise `false`.
///
pub fn is_valid_word(word: &str, word_list: &HashSet<String>) -> bool {
    word_list.contains(&word.to_lowercase()) // Check if word exists in the set
}

/// Displays the contents of specified text files.
/// 
/// # Parameters
/// - `files`: A slice of file contents (as strings) to display.
/// 
/// # Behavior
/// This function will print the content to the console. If a content is empty,
/// an error message is printed.
pub fn display_text(file_contents: &[&str]) {
    for content in file_contents.iter() {
        if !content.is_empty() {
            println!("{}", content);
        } else {
            println!("❌ Failed to load content");
        }
    }
}

// Embedding file contents directly into the binary
const WORD_LIST: &str = include_str!("../assets/word-list-raw.txt");
const SEVEN_UNIQUE_LETTER_WORDS: &str = include_str!("../assets/seven_unique_letter_words.txt");
const COMMANDS_TEXT: &str = include_str!("../assets/commands.txt");
const RULES_TEXT: &str = include_str!("../assets/rules.txt");
const INFO_TEXT: &str = include_str!("../assets/info.txt");
const GPL_LICENSE: &str = include_str!("../assets/gpl-3.0.md");

fn main() {
    // Load the word list from the embedded content
    let word_list = load_word_list(WORD_LIST);

    // Step 1: Generate letters
    let mut letters: Vec<char> = get_random_line(SEVEN_UNIQUE_LETTER_WORDS).unwrap().chars().collect();
    letters.shuffle(&mut rand::thread_rng());
    let required_letter = letters[0]; // First letter is required

    // Step 2: Prepare game state
    let mut used_words = HashSet::new();
    let mut total_score = 0;
    let mut message = "Welcome to Libre Bee!".to_string();
    println!("Libe Bee  Copyright (C) 2024  Zev Oster\nThis program comes with ABSOLUTELY NO WARRANTY; for details type `/warranty'.\nThis is free software, and you are welcome to redistribute it\nunder certain conditions; type /license for details.");

    // Step 3: Game loop
    loop {
        clear().expect("failed to clear screen");
        println!("Message: {}", message);
        println!("Score: {:?}", total_score);
        println!("🔤 Letters: {:?}", letters);
        println!("⭐ Required letter: '{}'", required_letter);
        println!("\nEnter a word (or type '/quit' to exit, '/help' for more commands and game rules, or another command):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let word = input.trim().to_lowercase();

        // Process commands
        if word.starts_with('/') {
            clear().expect("failed to clear screen");
            if word == "/quit" {
                break;
            } else if word == "/help" {
                display_text(&[INFO_TEXT, COMMANDS_TEXT, RULES_TEXT]);
            } else if word == "/info" {
                display_text(&[INFO_TEXT]);
            } else if word == "/rules" {
                display_text(&[RULES_TEXT]);
            } else if word == "/commands" {
                display_text(&[COMMANDS_TEXT]);
            } else if word == "/warranty" {
                println!("THERE IS NO WARRANTY FOR THE PROGRAM, TO THE EXTENT PERMITTED BY \nAPPLICABLE LAW. EXCEPT WHEN OTHERWISE STATED IN WRITING THE COPYRIGHT \nHOLDERS AND/OR OTHER PARTIES PROVIDE THE PROGRAM \"AS IS\" WITHOUT \nWARRANTY OF ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING, BUT NOT \nLIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR \nA PARTICULAR PURPOSE. THE ENTIRE RISK AS TO THE QUALITY AND \nPERFORMANCE OF THE PROGRAM IS WITH YOU. SHOULD THE PROGRAM PROVE \nDEFECTIVE, YOU ASSUME THE COST OF ALL NECESSARY SERVICING, REPAIR OR \nCORRECTION.")
            } else if word == "/license" {
                display_text(&[GPL_LICENSE]);
            } else if word == "/stats" {
                println!("Total score: {}", total_score);
                println!("Words used: {:?}", used_words);            
            } else if word == "/score" {
                println!("Total score: {}", total_score);
            } else if word == "/found" {
                println!("Words used: {:?}", used_words);
            } else {
                println!("❌ Invalid Command! Use '/commands' for a list of commands.");
            }
            println!("\nPress Enter to Continue...");
            let mut returnline = String::new();
            let _ = io::stdin().read_line(&mut returnline);
            continue;
        }

        // Validation checks
        if word.len() < 4 {
            message = "❌ Word too short!".to_string();
            continue;
        }

        if !word.contains(required_letter) {
            message = format!("❌ Must contain required letter '{}'", required_letter);
            continue;
        }

        if word.chars().any(|c| !letters.contains(&c)) {
            message = "❌ Invalid letters used!".to_string();
            continue;
        }

        if used_words.contains(&word) {
            message = "❌ Word already used!".to_string();
            continue;
        }

        if !is_valid_word(&word, &word_list) {
            message = "❌ Not a valid English word!".to_string();
            continue;
        }

        // Add word and compute score
        used_words.insert(word.clone());
        let points = compute_score(&word);
        total_score += points;

        message = format!("✅ '{}' is valid! You scored {} points.", word, points);
    }

    // Final Results
    println!("\n🎯 Game Over!");
    println!("Total score: {}", total_score);
    println!("Words used: {:?}", used_words);
}

#[cfg(test)]  // This ensures that these tests are only compiled when running tests
mod tests {
    use super::*;  // Import the functions being tested

    // Test for the compute_score function
    #[test]
    fn test_compute_score_pangram() {
        let word = "abcdefg";  // 7 unique characters
        let score = compute_score(word);
        assert_eq!(score, 7 + word.len());  // 7 for pangram + length of the word
    }

    #[test]
    fn test_compute_score_normal() {
        let word = "abcd";  // 4-letter word
        let score = compute_score(word);
        assert_eq!(score, 1);  // Special case for 4-letter words
    }

    #[test]
    fn test_is_valid_word() {
        let word_list = load_word_list(WORD_LIST);
        let word = "hello";  // Make sure this word exists in your word list
        assert!(is_valid_word(word, &word_list));
    }
}
