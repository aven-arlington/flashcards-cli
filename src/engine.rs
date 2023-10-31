pub mod config;
pub mod flashcard;
pub mod deck;
use crate::engine::deck::Deck;
use std::io::prelude::*;
use std::error::Error;
use log::{debug, log_enabled, Level};
use crate::Config;


pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    let mut deck = Deck::new(config.cards).expect("Could not create FlashCard Deck");

    if log_enabled!(Level::Debug) {
        debug!("Config object data");
        debug!("attempts_before_hint: {}", config.attempts_before_hint);
        debug!("attempts_before_wrong: {}", config.attempts_before_wrong);
        deck.print_cards();
    }

    println!("Welcome to FlashCards!");
    println!("Type \"quit\" to exit the application.");
    println!("Lets begin!");
    let mut quit = false;
    let mut consecutive_right: usize = 0;
    let mut wrong_count: usize = 0;
    while !quit && wrong_count < 3 {
        let card = deck.random_card_from_current_level_range().unwrap();
        print!("{}: ", card.clue_side);

        let mut input = String::new();
        std::io::stdout().flush().expect("Flush failed");
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let trimmed_input = input.trim();
        debug!("Input \"{}\"", trimmed_input);

        if trimmed_input == card.answer_side {
            consecutive_right += 1;
            println!("Correct! You have gotten {} right in a row!", consecutive_right);
        } else if trimmed_input == "quit" {
            println!("Quitting application");
            quit = true;
        } else {
            wrong_count += 1;
            println!("Incorrect, your input was \"{}\"", trimmed_input);
            debug!("Guess_count: {}", wrong_count);
        }

        if consecutive_right >= 5 {
            consecutive_right = 0;
            deck.increase_level_max();
        }
        
    }

    println!("Game Over");
    Ok(())
}
