use rand::seq::SliceRandom;
use rand::Rng;
use clearscreen::clear;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufRead};

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
pub fn load_word_list(file_path: &str) -> HashSet<String> {
    let mut word_set = HashSet::new();
    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(word) = line {
                word_set.insert(word.trim().to_string()); // Store each word
            }
        }
    }
    word_set
}

fn get_random_line(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
    let total_lines = lines.len();

    if total_lines == 0 {
        return Err(io::Error::new(io::ErrorKind::Other, "File is empty"));
    }

    let random_index = rand::thread_rng().gen_range(0..total_lines);
    Ok(lines[random_index].clone())
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
/// - `files`: A slice of file paths (as strings) to display.
/// 
/// # Behavior
/// This function will read each file, and print its content to the console. If a file cannot be loaded,
/// an error message is printed.
pub fn display_text(files: &[&str]) {
    for file in files.iter() {
        match fs::read_to_string(file) {
            Ok(content) => {
                println!("{}", content);
            }
            Err(_) => {
                println!("❌ Failed to load {}", file);
            }
        }
    }
}

fn main() {
    // Load the word list from the file
    let word_list = load_word_list("assets/word-list-raw.txt");

    // Step 1: Generate letters
    let mut letters: Vec<char> = get_random_line("assets/seven_unique_letter_words.txt").unwrap().chars().collect();
    letters.shuffle(&mut rand::thread_rng());
    let required_letter = letters[0]; // First letter is required

    // Step 2: Prepare game state
    let mut used_words = HashSet::new();
    let mut total_score = 0;
    let mut message = "Welcome to LibreBee!".to_string();

    // Step 3: Game loop
    loop {
        clear().expect("failed to clear screen");
        println!("Message: {}", message);
        println!("Score {:?}", total_score);
        println!("🔤 Letters: {:?}", letters);
        println!("⭐ Required letter: '{}'", required_letter);
        println!("\nEnter a word (or type '/quit' to exit, '/help' for more commands and game rules, or another command):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let word = input.trim().to_lowercase();

        // Process commands
        if word.starts_with('/') {
            if word == "/quit" {
                break;
            } else if word == "/help" {
                display_text(&["assets/commands.txt", "assets/rules.txt", "assets/info.txt"]);
            } else if word == "/info" {
                display_text(&["assets/info.txt"]);
            } else if word == "/rules" {
                display_text(&["assets/rules.txt"]);
            } else if word == "/commands" {
                display_text(&["assets/commands.txt"]);
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
        let word_list = load_word_list("assets/word-list-raw.txt");
        let word = "hello";  // Make sure this word exists in your word list
        assert!(is_valid_word(word, &word_list));
    }
}