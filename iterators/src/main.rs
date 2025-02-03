// Custom error enum
#[derive(Debug)]
enum MyError {
    EmptyInputError,
}

//  process input and handles errors
fn process_input(input: &str) -> Result<String, MyError> {
    if input.trim().is_empty() {
        Err(MyError::EmptyInputError)
    } else {
        Ok(format!("Processed input: {}", input))
    }
}

//  safely divide numbers using Option<T>
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

use std::collections::HashMap;
fn main() {
    let user_input = "Hello, Base Seoul!";
    match process_input(user_input) {
        Ok(content) => println!("{}", content),
        Err(MyError::EmptyInputError) => println!("Error: Input was empty!"),
    }
    let result1 = safe_divide(10.0, 2.0);
    let result2 = safe_divide(5.0, 0.0);

    match result1 {
        Some(value) => println!("Division successful: {}", value),
        None => println!("Cannot divide by zero!"),
    }

    match result2 {
        Some(value) => println!("Division successful: {}", value),
        None => println!("Cannot divide by zero!"),
    }

    // Working with Vec
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("Sum of numbers: {}", sum);

    let squared_even_numbers: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("Squared even numbers: {:?}", squared_even_numbers);

    // Working with HashMap
    let mut scores = HashMap::new();
    scores.insert("Alex", 100);
    scores.insert("Max", 85);

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    scores.entry("Charlie").or_insert(88);
    println!("Updated scores: {:?}", scores);
}

