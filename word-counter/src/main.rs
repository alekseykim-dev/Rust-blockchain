use std::collections::HashMap;

#[derive(Debug)]
enum MyError {
    EmptyInputError,
}
fn process_input(input: &str) -> Result<String, MyError> {
    if input.trim().is_empty() {
        Err(MyError::EmptyInputError)
    } else {
        Ok(format!("Processed input: {}", input))
    }
}
fn word_counter(text: &str) -> HashMap<String, usize> {
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());
        if !clean_word.is_empty() {
            *word_count.entry(clean_word.to_string()).or_insert(0) += 1;
        }
    }
    word_count
}
fn main() {
    let user_input = "Hello, Base Seoul! Welcome to Base. Hello Seoul!";
    match process_input(user_input) {
        Ok(content) => println!("{}", content),
        Err(MyError::EmptyInputError) => println!("Error: Input was empty!"),
    }

    // Word Counter Example
    let word_counts = word_counter(user_input);
    let total_words: usize = word_counts.values().sum();

    println!("Word Counts:");
    for (word, count) in &word_counts {
        println!("{}: {}", word, count);
    }
    println!("Total Number of Words: {}", total_words);
}