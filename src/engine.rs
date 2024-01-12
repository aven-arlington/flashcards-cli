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
            let mut attempts = config.attempts_before_wrong;
            while attempts > 0 {
                let input = get_input();
                debug!("Input \"{}\"", input);

                if input == "quit" {
                    println!("Quitting application");
                    return Ok(());
                } else if !(input == card.answer_side) {
                    attempts -= 1;
                    //println!("Incorrect, your input was: \"{}\"", input);
                    if attempts == 1{
                        println!("Incorrect. Try again");
                    } else {
                        println!("Incorrect. The correct answer is: \"{}\"", card.answer_side);
                    }
                } else {
                    println!(
                        "Correct! There are {} cards left in this hand.",
                        deck.hand_count()
                    );
                    break;
                }
            }
        }
        deck.add_level_to_deck();
    }
}

fn get_input() -> String {
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
    input
}

