use std::cmp::Ordering;
use std::io;
use colored::*;

mod helpers;

pub use crate::helpers::game_title::*;
pub use crate::helpers::secret_number::*;

// Use $ cargo doc --open to view the documentation

const MAX_NUMBER: u32 = 100;
const WRONG_NUMBER: u32 = 13;

fn main() {
    
    let secret_number = generate_secret_number(MAX_NUMBER, WRONG_NUMBER);
    let mut attempts = 0;

    show_game_title();

    loop {
        println!("Please input your guess or type 'quit' to exit.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim().to_lowercase() == "quit" {
            println!("{}", "Goodbye!".red());
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        if (guess < 1) || (guess > MAX_NUMBER) {
            let message = format!(
                "Please enter a number between 1 and {}. This attempt will not count 😅",
                MAX_NUMBER
            );
            println!("{}", message.yellow());
            continue;
        }

        attempts += 1;

        if guess == WRONG_NUMBER {
            println!("{}", "COMUNISTA!".red().bold());
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!(
                    "🎊 {} 🎊 It took you {} attempts",
                    "YOU WIN!".yellow().bold(),
                    attempts
                );
                break;
            }
        }
    }
}
