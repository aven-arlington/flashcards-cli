pub mod config;
pub mod parser;

use std::error::Error;
use crate::Config;
use crate::engine::parser::FlashCard;

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    println!("File_path: {}", config.file_path.as_path().display());
    let flashcards: Vec<FlashCard> = parser::parse_json(config.file_path)?;

    for card in flashcards {
        println!("Clue : {}", card.clue_side);
    }
    Ok(())
}

