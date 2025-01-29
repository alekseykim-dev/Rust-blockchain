struct Book {
    title: String,
    author: String,
}

fn main() {
    // Mutable References
    let mut data = String::from("Hello");
    modify_data(&mut data);
    println!("Modified data: {}", data);

    // Slices
    let text = String::from("Hello, Rustacean!");
    let slice = &text[7..];
    println!("Slice: {}", slice);

    // Lifetimes
    let string1 = String::from("Hello");
    let string2 = String::from("Base Seoul");
    let longest_str = longest(&string1, &string2);
    println!("Longest string: {}", longest_str);

    // Structs and Ownership
    let book = Book {
        title: String::from("Rust Programming"),
        author: String::from("Alex Kim"),
    };
    print_book(&book);
    println!("Book title: {}", book.title);

    // Collections
    let mut numbers = vec![1, 2, 3];
    numbers.push(4);
    print_vector(&numbers);

    // Closures and Borrowing
    let name = String::from("Base");
    let greet = || println!("Hello, {}!", name);
    greet();
}

fn modify_data(s: &mut String) {
    s.push_str(", mutable reference!");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn print_book(book: &Book) {
    println!("Title: {}, Author: {}", book.title, book.author);
}

fn print_vector(v: &Vec<i32>) {
    for num in v {
        println!("{}", num);
    }
}
