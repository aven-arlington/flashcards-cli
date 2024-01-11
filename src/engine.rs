pub mod config;
pub mod deck;
pub mod flashcard;
use crate::engine::deck::Deck;
use crate::Config;
use log::{debug, log_enabled, Level};
use std::error::Error;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut deck = Deck::new(config.cards);

    if log_enabled!(Level::Debug) {
        debug!("Config object data");
        debug!("attempts_before_hint: {}", config.attempts_before_hint);
        debug!("attempts_before_wrong: {}", config.attempts_before_wrong);
        deck.print_cards();
    }

    println!("Welcome to FlashCards!");
    println!("Type \"quit\" to exit the application.");
    loop {
        println!("Drawing a fresh hand of cards...");
        deck.draw_hand();
        while let Some(card) = deck.next_card() {
            print!("{}: ", card.clue_side);

            if let Some(input) = get_input() {
                debug!("Input \"{}\"", input);

                if input == card.answer_side {
                    println!(
                        "Correct! There are {} cards left in this hand.",
                        deck.hand_count()
                    );
                } else {
                    println!("Incorrect, your input was: \"{}\"", input);
                    println!("The correct answer was: \"{}\"", card.answer_side);
                }
            } else {
                println!("Quitting application");
                return Ok(());
            }
        }
        deck.add_level_to_deck();
    }
}

fn get_input() -> Option<String> {
    let mut input = String::new();
    std::io::stdout().flush().expect("Flush failed");
    while std::io::stdin().read_line(&mut input).is_ok() {
        input = input.trim().to_string();
        if !input.is_empty() {
            break;
        }
        input.clear();
        println!("Please enter a value");
    }
    match input.as_str() {
        "quit" => None,
        _ => Some(input),
    }
}

