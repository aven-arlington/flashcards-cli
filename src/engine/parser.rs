// JSON file parsing code for flashcards
use crate::engine::flashcard::FlashCard;
use std::path::Path;
use std::collections::BTreeMap;
use serde_json;
use std::fs;
use log::debug;

pub fn parse_json<P: AsRef<Path>>(file_path:P) -> Result<BTreeMap<u32, Vec<FlashCard>>, &'static str> {
    let file_data = fs::read_to_string(file_path).expect("Unable to read json file");

    debug!("File data: {}", file_data);

    let mut flashcards: Vec<FlashCard> = serde_json::from_str(file_data.as_str()).expect("Unable to parse data string");
    if flashcards.is_empty() {
        return Err("No FlashCards in json file");
    }
    flashcards.sort();

    let mut level_map: BTreeMap<u32, Vec<FlashCard>> = BTreeMap::new();
    for card in flashcards {
        level_map.entry(card.level)
            .and_modify(|level| level.push(card.clone()))
            .or_insert(vec![card.clone()]);
    }
    Ok(level_map)
}

