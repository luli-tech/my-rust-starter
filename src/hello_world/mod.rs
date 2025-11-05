use std::io::{self, Write};

pub fn run() {
    println!("Hello, world!");
    print!("please make your input here: ");
    io::stdout().flush().unwrap(); // ensures the prompt appears before input
    let mut put: String = String::new();
    io::stdin().read_line(&mut put).expect("expected a string");
   
    println!("you entered: {}", put)
}