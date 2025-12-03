mod stats;

use stats::get_num_chars;
use stats::get_num_words;
use std::fs;

fn main() {
    let file_path = String::from("books/frankenstein.txt");
    let contents = get_book_text(file_path);
    let num_words = get_num_words(contents.clone());
    let num_chars = get_num_chars(contents);

    println!("Found {num_words} total words");
    for (character, count) in num_chars {
        println!("{}: {}", character, count);
    }
}

fn get_book_text(path: String) -> String {
    fs::read_to_string(path).expect("Could not read file at path")
}
