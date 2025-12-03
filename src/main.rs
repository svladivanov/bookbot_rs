use std::fs;

fn main() {
    let file_path = String::from("books/frankenstein.txt");
    let contents = get_book_text(file_path);
    let num_words = get_num_words(contents);
    println!("Found {num_words} total words");
}

fn get_book_text(path: String) -> String {
    fs::read_to_string(path).expect("Could not read file at path")
}

fn get_num_words(text: String) -> usize {
    text.split_whitespace().count()
}
