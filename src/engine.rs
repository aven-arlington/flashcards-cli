pub mod config;
pub mod flashcard;
pub mod deck;
use crate::engine::deck::Deck;
use std::io::prelude::*;
use std::error::Error;
use log::{debug, log_enabled, Level};
use crate::Config;


pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    debug!("Parsing json file: {}", config.file_path.as_path().display());
    let mut deck = Deck::new(config.file_path).expect("Could not create FlashCard Deck");

    if log_enabled!(Level::Debug) {
        deck.print_cards();
    }

    println!("Welcome to FlashCards!");
    println!("Type \"quit\" to exit the application.");
    println!("Lets begin!");
    let mut quit = false;
    let mut score: usize = 0;
    let mut wrong_count: usize = 0;
    while !quit && wrong_count < 3 {
        let card = deck.random_card().unwrap();
        print!("{}: ", card.clue_side);

        let mut input = String::new();
        std::io::stdout().flush().expect("Flush failed");
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let trimmed_input = input.trim();
        debug!("Input \"{}\"", trimmed_input);

        if trimmed_input == card.answer_side {
            score += 1;
            println!("Correct! Current score: {}", score);
        } else if trimmed_input == "quit" {
            println!("Quitting application");
            quit = true;
        } else {
            wrong_count += 1;
            println!("Incorrect, your input was \"{}\"", trimmed_input);
            debug!("Guess_count: {}", wrong_count);
        }
        
    }

    println!("Game Over");
    Ok(())
}
