fn main() {
    // Ownership
    let owner = String::from("Hello, Rust!"); 
    print_owner(owner);

    // Uncommenting the next line will cause a compile error because `owner` no longer owns the data
    // println!("{}", owner);

    let borrower = String::from("Borrow me");
    print_borrow(&borrower); 
    println!("Still usable: {}", borrower);
}

fn print_owner(s: String) {
    println!("I own this now: {}", s);
    // `s` goes out of scope here, and the memory is freed
}

fn print_borrow(s: &String) {
    println!("I just borrowed this: {}", s);
    // Since this is a reference, nothing is freed
}
