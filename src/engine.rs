pub mod config;
pub mod parser;
use std::io::prelude::*;
use std::error::Error;
use std::collections::BTreeMap;
use log::{debug, error, log_enabled, info, Level};
use parser::FlashCard;
use rand::prelude::*;
use crate::Config;


pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    debug!("Parsing json file: {}", config.file_path.as_path().display());
    let levels = parser::parse_json(config.file_path)?;

    // TODO Remove Debug Prints 
    print_cards(&levels);
    // End Debug Prints

    println!("Welcome to FlashCards!");
    println!("Type \"Quit\" to exit the application.");
    let mut level_iter = levels.iter();
    while let Some((key, level)) = level_iter.next() {
        println!("Starting level: {}", key);
        for card in level {
            let mut guess_count: usize = 0;
            print!("{}: ", card.clue_side);
            std::io::stdout().flush().expect("Flush failed");
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read input");
                let trimmed_input = input.trim();
                debug!("Input:{}:", trimmed_input);
                debug!("Guess_count: {}", guess_count);
                if trimmed_input == card.answer_side {
                    println!("Correct");
                    break;
                } else  if trimmed_input == "quit" {
                    println!("Quitting application");
                    return Ok(());
                } else {
                    guess_count += 1;
                    println!("Incorrect, your input was \"{}\"", card.answer_side);
                    debug!("Guess_count: {}", guess_count);
                    continue;
                }
            }
        }
    }

    Ok(())
}

pub fn print_cards(levels:&BTreeMap<u32, Vec<FlashCard>>) {
    let level_iter = levels.iter();
    for (key, level) in level_iter {
        debug!("Level {}:", key);
        for card in level {
            debug!("{}", card.clue_side);
        }
    }
}
