pub mod config;
pub mod parser;
pub mod flashcard;
use crate::engine::flashcard::FlashCard;
use std::io::prelude::*;
use std::error::Error;
use log::{debug, error, log_enabled, info, Level};
use rand::prelude::*;
use crate::Config;


pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    debug!("Parsing json file: {}", config.file_path.as_path().display());
    let cards = parser::parse_json(config.file_path)?;

    print_cards(&cards);

    println!("Welcome to FlashCards!");
    println!("Type \"Quit\" to exit the application.");
    let mut cards_iter = cards.iter();
    let mut quit = false;
    let mut score: usize = 0;
    let mut guess_count: usize = 0;
    println!("Lets begin!");
    while !quit || guess_count < 3 {
        let card = cards_iter.next().unwrap();
        print!("{}: ", card.clue_side);
        let mut input = String::new();
        std::io::stdout().flush().expect("Flush failed");
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let trimmed_input = input.trim();
        debug!("Input \"{}\"", trimmed_input);
        debug!("Guess_count: {}", guess_count);
        if trimmed_input == card.answer_side {
            score += 1;
            println!("Correct! Current score: {}", score);
        } else  if trimmed_input == "quit" {
            println!("Quitting application");
            quit = true;
        } else {
            guess_count += 1;
            println!("Incorrect, your input was \"{}\"", trimmed_input);
            debug!("Guess_count: {}", guess_count);
        }
    }

    Ok(())
}

pub fn print_cards(cards:&Vec<FlashCard>) {
    for card in cards {
        debug!("{}", card.clue_side);
    }
}
