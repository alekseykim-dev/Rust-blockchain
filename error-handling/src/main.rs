
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

fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}
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
}