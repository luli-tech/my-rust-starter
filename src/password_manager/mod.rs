use clap::{Parser, Subcommand};
use std::path::PathBuf;
use colored::*;
use rpassword::read_password;

mod manager;
use manager::PasswordManager;

#[derive(Parser)]
#[command(name = "password-manager")]
#[command(about = "A secure password manager with encryption", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new password
    Add {
        /// Service name (e.g., "github.com")
        service: String,
        /// Username or email
        username: String,
    },
    /// Retrieve a password
    Get {
        /// Service name
        service: String,
        /// Username or email
        username: String,
    },
    /// List all stored services and usernames
    List,
    /// Delete a stored password
    Delete {
        /// Service name
        service: String,
        /// Username or email
        username: String,
    },
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Get master password
    print!("{}", "Enter master password: ".yellow());
    std::io::stdout().flush().unwrap();
    let master_password = read_password()?;

    // Initialize database in the user's home directory
    let mut db_path = dirs::home_dir().expect("Could not find home directory");
    db_path.push(".password_manager.db");

    let manager = PasswordManager::new(&master_password, db_path)?;

    match cli.command {
        Commands::Add { service, username } => {
            print!("{}", "Enter password to store: ".yellow());
            std::io::stdout().flush().unwrap();
            let password = read_password()?;
            manager.add_password(&service, &username, &password)?;
        }
        Commands::Get { service, username } => {
            match manager.get_password(&service, &username) {
                Ok(password) => println!("Password: {}", password.bright_green()),
                Err(e) => println!("{}", format!("Error: {}", e).red()),
            }
        }
        Commands::List => {
            let services = manager.list_services()?;
            if services.is_empty() {
                println!("{}", "No passwords stored yet.".yellow());
            } else {
                println!("{}", "Stored passwords:".cyan().bold());
                for (service, username) in services {
                    println!("{}: {}", service.blue(), username.green());
                }
            }
        }
        Commands::Delete { service, username } => {
            manager.delete_password(&service, &username)?;
        }
    }

    Ok(())
}