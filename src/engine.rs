pub mod config;
pub mod parser;
pub mod flashcard;
use crate::engine::flashcard::FlashCard;
use std::io::prelude::*;
use std::error::Error;
use log::{debug, log_enabled, Level};
use rand::prelude::*;
use crate::Config;


pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    debug!("Parsing json file: {}", config.file_path.as_path().display());
    let cards = parser::parse_json(config.file_path)?;

    if log_enabled!(Level::Debug) {
        print_cards(&cards);
    }

    println!("Welcome to FlashCards!");
    println!("Type \"quit\" to exit the application.");
    let mut quit = false;
    let mut score: usize = 0;
    let mut wrong_count: usize = 0;
    let mut rng = rand::thread_rng();
    println!("Lets begin!");
    let mut card_index = rng.gen_range(0..cards.len());
    while !quit && wrong_count < 3 {
        print!("{}: ", cards[card_index].clue_side);

        let mut input = String::new();
        std::io::stdout().flush().expect("Flush failed");
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let trimmed_input = input.trim();
        debug!("Input \"{}\"", trimmed_input);

        if trimmed_input == cards[card_index].answer_side {
            score += 1;
            card_index = rng.gen_range(0..cards.len());
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

pub fn print_cards(cards:&Vec<FlashCard>) {
    for card in cards {
        println!("{}", card.clue_side);
    }
}
