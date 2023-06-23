use rand::Rng;
use std::io;

fn get_guess() -> String {
    println!("Guess the number");

    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess
}

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("-> secret is {secret_number}");

    let current_guess = get_guess();
    println!("your guess is {current_guess}");
}
