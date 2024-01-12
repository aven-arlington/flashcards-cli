pub mod config;
pub mod deck;
pub mod flashcard;
use crate::engine::deck::Deck;
use crate::engine::flashcard::FlashCard;
use crate::engine::config::Config;
use log::{debug, log_enabled, Level};
use std::error::Error;
use std::fmt;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct QuitApp;

impl fmt::Display for QuitApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quitting application")
    }
}

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
        deck.draw_hand();
        println!("Drawing a fresh hand of {} cards...", deck.hand_count());
        while let Some(card) = deck.next_card() {
            if show_card(&card, config.attempts_before_wrong).is_err() {
                println!("Quitting application");
                return Ok(());
            }
        }
        deck.add_level_to_deck();
    }
}

fn show_card(card: &FlashCard, mut attempts: usize) -> Result<bool, QuitApp> {
    print!("{}: ", card.clue_side);
    loop {
        match get_input().as_str() {
            "quit" => return Err(QuitApp),
            s if s == card.answer_side => {
                println!("Correct!",);
                return Ok(true);
            }
            _ => {
                attempts -= 1;
                if attempts >= 1 {
                    println!("Incorrect. Try again");
                    print!("{}: ", card.clue_side);
                } else {
                    println!(
                        "Incorrect. The correct answer was: \"{}\"",
                        card.answer_side
                    );
                    return Ok(false);
                }
            }
        }
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
        println!("Please provide an answer or \"quit\" to exit the application.");
    }
    input
}

