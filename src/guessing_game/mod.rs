use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

pub fn run() {
    println!("{}", "Welcome to the Guessing Game!".bright_green().bold());
    println!("{}", "I'm thinking of a number between 1 and 100...".cyan());

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("{}", "Please input your guess: ".yellow());
        io::stdout().flush().unwrap();
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a valid number!".red().bold());
                continue;
            }
        };

        println!("You guessed: {}", guess.to_string().blue());

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", " You win! ".bright_green().bold());
                break;
            }
        }
    }
}