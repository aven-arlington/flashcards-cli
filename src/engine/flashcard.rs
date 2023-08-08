use std::cmp::Ordering;
use serde::Deserialize;

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


