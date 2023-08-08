use crate::engine::flashcard::FlashCard;
use std::path::Path;
use serde_json;
use std::fs;
use log::debug;

pub fn parse_json<P: AsRef<Path>>(file_path:P) -> Result<Vec<FlashCard>, &'static str> {
    let file_data = fs::read_to_string(file_path).expect("Unable to read json file");

    debug!("File data: {}", file_data);

    let mut flashcards: Vec<FlashCard> = serde_json::from_str(file_data.as_str()).expect("Unable to parse data string");
    if flashcards.is_empty() {
        return Err("No FlashCards in json file");
    }
    flashcards.sort();
    Ok(flashcards)
}

