pub mod config;
pub mod parser;
use std::error::Error;
use std::collections::BTreeMap;
use crate::Config;
use crate::engine::parser::FlashCard;

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    println!("File_path: {}", config.file_path.as_path().display());
    let levels = parser::parse_json(config.file_path)?;
    
    for (level_number, cards) in levels {
        for card in cards {
            println!("Clue: {}", card.clue_side);
        }
    }
    Ok(())
}
