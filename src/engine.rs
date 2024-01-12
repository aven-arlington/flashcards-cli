mod deck;
pub mod flashcard;
use std::error::Error;
use std::io::prelude::*;

pub fn run(config: crate::Config) -> Result<(), Box<dyn Error>> {
    let mut deck = deck::Deck::new(config.cards);

    println!("Welcome to FlashCards!");
    println!("Type \"quit\" to exit the application.");
    loop {
        deck.draw_hand();
        println!("Drawing a fresh hand of {} cards...", deck.hand_count());
        while let Some(card) = deck.next_card() {
            if show_card(&card, config.attempts_before_wrong).is_none() {
                println!("Quitting application");
                return Ok(());
            }
        }
        deck.increase_level_pool();
    }
}

fn show_card(card: &crate::FlashCard, mut attempts: usize) -> Option<bool> {
    print!("{}: ", card.clue_side);
    loop {
        match get_input().as_str() {
            "quit" => return None,
            s if s == card.answer_side => {
                println!("Correct!",);
                return Some(true);
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
                    return Some(false);
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

