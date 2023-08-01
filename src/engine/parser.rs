// JSON file parsing code for flashcards
use std::path::Path;
use std::cmp::Ordering;
use serde::Deserialize;
use std::collections::BTreeMap;
use serde_json;
use std::fs;

#[derive(Deserialize, Debug, Eq, Clone)]
pub struct FlashCard {
    pub clue_side: String,
    pub answer_side: String,
    pub hint: String,
    pub level: u32
}

impl Ord for FlashCard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.level.cmp(&other.level)
    }
}

impl PartialOrd for FlashCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FlashCard {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level
    }
}

pub fn parse_json<P: AsRef<Path>>(file_path:P) -> Result<BTreeMap<u32, Vec<FlashCard>>, &'static str> {
    let file_data = fs::read_to_string(file_path).expect("Unable to read json file");

    // TODO Remove debug print
    println!("File data: {}", file_data);

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

