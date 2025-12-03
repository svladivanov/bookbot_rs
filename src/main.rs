mod stats;

use stats::get_num_chars;
use stats::get_num_words;
use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: bookbot_rs <file_path> from the root dir");
        exit(1);
    }
    let file_path = &args[1];

    let contents = get_book_text(file_path.clone());
    let num_words = get_num_words(contents.clone());
    let num_chars = get_num_chars(contents);

    let mut sorted_chars: Vec<(&char, &i32)> = num_chars.iter().collect();
    sorted_chars.sort_by(|a, b| b.1.cmp(a.1));

    println!("============ BOOKBOT ============");
    println!("Analyzing book found at {file_path}...");
    println!("----------- Word Count ----------");
    println!("Found {num_words} total words");
    println!("--------- Character Count -------");
    for (character, count) in sorted_chars {
        if character.is_alphanumeric() {
            println!("{}: {}", character, count);
        }
    }
    println!("============= END ===============");
}

fn get_book_text(path: String) -> String {
    fs::read_to_string(path).expect("Could not read file at path")
}
