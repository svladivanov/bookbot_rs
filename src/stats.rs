use std::collections::HashMap;

pub fn get_num_words(text: String) -> usize {
    text.split_whitespace().count()
}

pub fn get_num_chars(text: String) -> HashMap<char, i32> {
    let mut counts: HashMap<char, i32> = HashMap::new();

    for c in text.to_lowercase().chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    counts
}
