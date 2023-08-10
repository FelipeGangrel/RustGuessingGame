use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Use $ cargo doc --open to view the documentation

const MAX_NUMBER: u32 = 100;
const WRONG_NUMBER: u32 = 13;

fn generate_secret_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=MAX_NUMBER);

    if (secret_number == WRONG_NUMBER) {
        return generate_secret_number();
    }

    return secret_number;
}

fn main() {
    let game_title = "
    ⠄⠄⠄⢀⣀⣤⣤⣤⣤⣤⣤⣄⣀⠄⠄⠄⠄⠄⠄⠄⣀⣀⣤⣄⡀⠄⠄⠄⠄⠄
    ⠄⠄⣴⣿⣿⣉⣿⣿⡿⠿⠿⠿⠿⢿⣶⣦⡀⢠⣾⣿⣿⣿⣿⣿⣿⣦⡀⠄⠄⠄
    ⠄⠐⣿⡿⠟⣛⣩⣥⣶⣶⣶⣶⣶⣶⣭⣍⣓⢹⣯⣥⣶⣶⣶⣶⣦⣽⣿⡄⠄⠄
    ⣠⣾⡟⣿⣿⣿⣿⠿⢛⣫⣭⣭⣭⣭⣝⡛⢿⠎⢋⣩⣭⣭⣭⣭⣭⣍⠙⠛⡄⠄
    ⣿⣿⠇⢹⣿⢟⣵⣾⣿⣿⣿⡿⠛⠛⣿⣿⣦⠘⣿⣿⣿⣿⣿⣿⣿⠟⠛⢿⣶⣆
    ⣿⣿⣶⣿⣿⢸⣿⣿⣿⣿⣿⣧⣅⣴⣿⣿⠟⡘⠻⢿⣿⣿⣿⣿⣿⣷⣥⣾⡿⢛
    ⢻⣿⣿⣿⣿⣷⣍⣛⣛⣛⣛⣛⣛⣫⣵⣶⣿⢋⣾⣶⣦⣬⣭⣭⣭⣭⣭⠴⠖⠋
    ⣾⣿⣿⣿⣿⣿⣿⣶⣭⣭⣭⣭⣥⣶⣾⠟⣱⣿⣷⣦⣩⣉⣉⣉⣭⣶⣶⣷⠄⠄
    ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠿⢟⣛⣛⣅⠄
    ⣿⣿⣿⡿⢏⣭⣭⣭⣉⣙⣛⣛⣛⣋⣉⣩⣭⣥⣶⣶⣶⠶⠶⠾⣛⣛⣛⡻⠃⠄
    ⣿⣿⣿⣷⡜⢿⣿⣭⠭⠭⠭⣭⣭⣭⣭⣥⣶⣶⣶⣶⣶⣿⡿⠿⣛⡹⠛⠁⠄⠄
    ⠿⠿⠿⣧⣜⣷⣶⣶⣿⠿⣷⣶⣦⣤⣭⣭⣭⣭⣤⣤⣤⣴⠾⠟⠃⠄⠄⠄⠄⠄
    ⢀⣴⣷⣶⣶⣶⣶⣭⣭⣭⣤⣒⣒⣒⣒⣒⣛⣩⣭⣭⣶⣶⣤⡀⠄⠄⠄⠄⠄⠄
    ⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣆⠄⠄⠄⠄⠄
    ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⠄⠄⠄⠄
    .⣿⣿⣿⣿⣿ GUESS THE NUMBER ⣷⣿⣷⣿⣷ 
    ";

    println!("{}", game_title.green());

    // Generate a random number from 1 to 100
    // The lower bound is inclusive and the upper bound is exclusive
    // If you wanted to include the upper bound, you would use `..=`
    // Example: `1..=100`
    let secret_number = generate_secret_number();
    let mut attempts = 0;

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

        if (guess == WRONG_NUMBER) {
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
