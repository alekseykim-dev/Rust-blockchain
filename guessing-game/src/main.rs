use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number from 1 to 20 and win 0.005 ETH!");
    println!("Each incorrect guess costs 0.001 ETH");

    let secret_number = rand::thread_rng().gen_range(1..=20);

    loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {guess}");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small! 0.001 ETH deducted from your wallet"),
        Ordering::Greater => println!("Too big! 0.001 ETH deducted from your wallet"),
        Ordering::Equal => { println!("You win! 0.005 ETH was added to your wallet");
        break;}
     }
    }
}