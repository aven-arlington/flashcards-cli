pub mod config;
pub mod parser;
use std::io;
use std::error::Error;
use std::collections::BTreeMap;
use parser::FlashCard;
use rand::prelude::*;
use crate::Config;

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    println!("Parsing json file: {}", config.file_path.as_path().display());
    let levels = parser::parse_json(config.file_path)?;

    // TODO Remove Debug Prints 
    print_cards(&levels);
    // End Debug Prints

    let mut level_iter = levels.iter();
    while let Some((key, level)) = level_iter.next() {
        println!("Starting level: {}", key);
        for card in level {
            let mut guess_count: usize = 0;
            println!("Flashcard: {}", card.clue_side);
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                println!("Input:{}:", input.trim());
                println!("Guess_count: {}", guess_count);
                if input.trim() == card.answer_side {
                    println!("Correct");
                    break;
                } else {
                    guess_count += 1;
                    println!("Incorrect, your input was \"{}\"", card.answer_side);
                    println!("Guess_count: {}", guess_count);
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
        println!("Level {}:", key);
        for card in level {
            println!("{}", card.clue_side);
        }
    }
}
