mod hello_world;
mod guessing_game;
mod password_manager;

use colored::*;

fn main() {
    println!("{}", "Choose a program to run:".cyan().bold());
    println!("{}. {}", "1".green(), "Hello World Example");
    println!("{}. {}", "2".green(), "Guessing Game");
    println!("{}. {}", "3".green(), "Password Manager");
    print!("{}", "Enter your choice (1-3): ".yellow());
    
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => hello_world::run(),
        "2" => guessing_game::run(),
        "3" => {
            if let Err(e) = password_manager::run() {
                eprintln!("{}", format!("Error: {}", e).red());
            }
        },
        _ => println!("{}", "Invalid choice!".red())
    }
}
