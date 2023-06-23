use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn get_guess(count: i32) -> Result<i32, bool> {
    println!("Please input your {count}-th guess.");
    let mut guess = String::new();
    match io::stdin().read_line(&mut guess) {
        Ok(_) => {
            let guess = guess.trim();
            if guess.to_lowercase() == "quit" {
                Err(false)
            } else {
                match guess.parse() {
                    Ok(result) => Ok(result),
                    Err(_) => Err(true),
                }
            }
        }
        Err(_) => Err(true),
    }
}

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("-> secret is {secret_number}");

    let mut count = 1;
    loop {
        let guess = get_guess(count);
        match guess {
            Ok(current_guess) => {
                let res = current_guess.cmp(&secret_number);

                println!(
                    "your {count}-th guess is {current_guess}, {}",
                    match res {
                        Ordering::Less => "too low",
                        Ordering::Equal => "you win",
                        Ordering::Greater => "too high",
                    }
                );

                if res == Ordering::Equal {
                    break;
                }
            }
            Err(true) => continue,
            Err(false) => break,
        }
        count += 1;
    }
}
