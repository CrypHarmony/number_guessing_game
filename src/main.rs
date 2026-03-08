// A number guessing game in Rust programming language 




// Import necessary crates and modules
use rand::Rng;
use std::{io::{self, Write}, cmp::Ordering::{Less, Greater, Equal}};
use colored::*;

fn main() {
// Generate a random number beetwen 1 and 100
    let mut rng = rand::thread_rng();
    let secret_number: u8 = rng.gen_range(1..=100);
    // Display the  welcome message and  difficulty options
    println!("{}", "\t╔═════════════════════════════════════════════════════╗".blue().bold());
    println!("{}", "\t║            🎯 Welcome to the Number Guess! 🎯       ║".blue().bold());
    println!("{}", "\t╚═════════════════════════════════════════════════════╝".blue().bold());
    
    println!("\n{}", "🔧 Please select a difficulty level: ".bright_yellow().bold());
    println!("{}", "1) 🟢 Easy (10 chances)".green().bold());
    println!("{}", "2) 🟡 Medium (5 chances)".yellow().bold());
    println!("{}", "3) 🔴 Hard (3 chance)".red().bold());
// Loop until the user select a valid difficulty level
    let difficulty = loop {
    
     let mut input = String::new();
        print!("{}", "👉 Enter 1 for Easy, 2 for Medium, or 3 for Hard: ".cyan().bold());

        if io::stdout().flush().is_err() {
            println!("{}", "⚠️  Failed to flush stdout.".red().bold());
            continue;
        }
        // Read the user input and handle potential errors
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(_) => {
                println!("{}", "⚠️  Failed to read input.".red().bold());
                continue;
            }
        }
        // Match the input to determine the selected difficulty level, and handle invalid selections
        match input.trim() {
            "1" => break "easy",
            "2" => break "medium",
            "3" => break "hard",
            _ => {
                println!("{}", "⛔ Invalid selection. Please select 1, 2, or 3.".red().bold().underline());
                continue;
            }
        }
    };
    // Set the number of attempts based on the selected difficulty level
    let attempts = match difficulty {
        "easy" => 10,
        "medium" => 5,
        "hard" => 3,
        _ => 1
    };

    println!("\n{}", format!("🌟 You selected '{}' mode! You have {} attempts.", difficulty, attempts).bright_cyan().bold().italic());
    
    let mut remaining_attempts = attempts;
    
    loop {
         
        let mut input = String::new();
        print!("{}", "👉 Enter your guess: ".yellow().bold());

        if io::stdout().flush().is_err() {
            println!("{}", "⚠️  Failed to flush stdout.".red().bold());
            continue;
        }
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(_) => {
                println!("{}", "⚠️  Failed to read input.".red().bold());
                continue;
            }
        }
        // Parse the user input and handle potential errors
        let parsed_input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "⛔ Please enter a valid integer.".red().bold().underline());
                continue;
            }
        };
        // Decrease the remaining attempts and compare the user's guess with the secret number, providing feedback accordingly
        remaining_attempts -= 1;
        // Match the user's guess with the secret number
        match parsed_input.cmp(&secret_number) {
            Less if remaining_attempts > 0 => println!("{}", format!("⬇️  Too low! Try again, {} attempts remaining!", remaining_attempts).cyan().bold().italic()),
            Greater if remaining_attempts > 0 => println!("{}", format!("⬆️  Too high! Try again, {} attempts remaining!", remaining_attempts).magenta().bold().italic()),
            Equal => {
                println!("\n{}", format!("🎉 Congratulations! You guessed the right number: {}", secret_number).green().bold().underline());
                println!("{}", "🌟 Amazing! You're a Number Guessing Master! 🌟".green().bold());
                break;
            },
            _ => {}
        }
// Check if the user has run out of attempts and provide a game over message
        if remaining_attempts == 0 {
            println!("\n{}", "💀 You ran out of chances! Better luck next time!".red().bold());
            println!("{}", "🤡 Haha, you really thought you'd win? Too bad!".red().bold().italic());
            break;
        }

    }

    println!("\n{}", "👋 Thanks for playing! See you next time!".white().italic().dimmed());
    println!("{}", "══════════════════════════════════════════".blue().bold());
}



// Key Conceptsfor this program include:

// We used Rust libraries such as 'rand' for generating random numbers, 'std::io' for handling user input and output, and 'colored' for adding color
// to the terminal output. The program generates a random number between 1 and 100 and prompts the user to select a difficulty level, which determines 
// the number of attempts they have to guess the number.
// In the main function, we created a secret number using the 'rand' crate and displayed a welcome message along with difficulty options.
// We used a loop to ensure the user selects a valid difficulty level and set the number of attempts based on the user's choice.
// The game then enters a loop where it prompts the user to enter their guess, checks if the input is valid, and compares it to the secret number.
// We matched the user's input with the secret number and provided feedback on whether the guess was too low, too high, or correct.
// If the user runs out of attempts, a game over message is displayed. Finally, we thank the user for playing and end the program 
