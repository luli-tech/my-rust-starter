mod hello_world;
mod guessing_game;

fn main() {
    println!("1. Hello World Example");
    println!("2. Guessing Game");
    print!("Choose a program to run (1 or 2): ");
    
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => hello_world::run(),
        "2" => guessing_game::run(),
        _ => println!("Invalid choice!")
    }
}
