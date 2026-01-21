use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    println!("Guess a number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {guess}");
    
    let guess: u32 = guess.trim().parse().expect("Please type in a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }

    println!("The secret number is {}", secret_number);
}
