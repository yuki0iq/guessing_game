use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn get_guess() -> i32 {
    println!("Guess the number");

    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess.trim().parse().expect("Please type a number!")
}

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("-> secret is {secret_number}");

    let current_guess = get_guess();
    println!(
        "your guess is {current_guess}, {}",
        match current_guess.cmp(&secret_number) {
            Ordering::Less => "too low",
            Ordering::Equal => "you win",
            Ordering::Greater => "too high",
        }
    );
}
