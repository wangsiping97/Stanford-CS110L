// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let word_len = secret_word.len();
    let mut word_so_far_chars: Vec<char> = Vec::new();
    for _ in 0..word_len {
        word_so_far_chars.push('-');
    }
    let mut guess_so_far_chars: Vec<char> = Vec::new();

    println!("Welcome to CS110L Hangman!");

    let mut incorrent_guess: u32 = 0;
    let mut corrent_chars: usize = 0;
    while incorrent_guess < NUM_INCORRECT_GUESSES {

        if corrent_chars == word_len {
            println!("Congratulations you guessed the secret word: {}!", secret_word);
            return;
        }
        println!("The word so far is {}", word_so_far_chars.iter().collect::<String>());
        println!("You have guessed the following letters: {}", guess_so_far_chars.iter().collect::<String>());
        println!("You have {} guesses left", NUM_INCORRECT_GUESSES - incorrent_guess);
        print!("Please guess a letter: ");

        // get input from stdin
        let mut buffer = String::new();
        let _ = io::stdout().flush().expect("Flushing error");
        io::stdin().read_line(&mut buffer).expect("Did not enter a correct string");
        // get input char
        let input_char = buffer.chars().nth(0).expect("");
        guess_so_far_chars.push(input_char);

        let mut is_correct = false;

        for i in 0..word_len {
            if secret_word_chars[i] == input_char {
                is_correct = true;
                corrent_chars += 1;
                word_so_far_chars[i] = input_char;
            }
        }

        if !is_correct {
            incorrent_guess += 1;
        }
        println!();

    }

    println!("Sorry, you ran out of guesses!");

}
