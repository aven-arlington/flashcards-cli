// JSON file parsing code for flashcards
use std::path::PathBuf;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
pub struct FlashCard {
    pub clue_side: String,
    pub answer_side: String,
    pub hint: String
}

pub fn parse_json(file_path:PathBuf) -> Result<Vec<FlashCard>, &'static str> {
        let mut flashcards: Vec<FlashCard> = Vec::new();
        Ok(flashcards)
    }

