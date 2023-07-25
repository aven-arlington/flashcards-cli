// JSON file parsing code for flashcards
use std::path::Path;
use serde::Deserialize;
use serde_json;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct FlashCard {
    pub clue_side: String,
    pub answer_side: String,
    pub hint: String,
    pub level: String
}

pub fn parse_json<P: AsRef<Path>>(file_path:P) -> Result<Vec<FlashCard>, &'static str> {
    let file_data = fs::read_to_string(file_path).expect("Unable to read json file");
    println!("File data: {}", file_data);
    let flashcards: Vec<FlashCard> = serde_json::from_str(file_data.as_str()).expect("Unable to parse data string");
    Ok(flashcards)
}

