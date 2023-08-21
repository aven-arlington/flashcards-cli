use crate::engine::flashcard::FlashCard;
use std::path::Path;
use std::fs;
use log::debug;
use rand::prelude::*;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<FlashCard>,
    rng: ThreadRng
}

impl Deck {
    pub fn new<P: AsRef<Path>>(file_path:P) -> Result<Self, &'static str> {
        let file_data = fs::read_to_string(file_path).expect("Unable to read config file");

        debug!("File data: {}", file_data);

        let mut flashcards: Vec<FlashCard> = serde_yaml::from_str(file_data.as_str()).expect("Unable to parse data string");
        if flashcards.is_empty() {
            return Err("No FlashCards in config file");
        }
        flashcards.sort();
        
        Ok(Self { 
            cards:flashcards,
            rng:rand::thread_rng(),
        })
    }

    pub fn next(&self) -> Result<FlashCard, &'static str> {
        Ok(self.cards.get(0).expect("No Flashcards in deck").clone())
    }

    pub fn print_cards(&self) {
        for card in &self.cards {
            debug!("{}", card.clue_side);
        }
    }

    pub fn random_card(&mut self) -> Result<FlashCard, &'static str> {
        let card_index = self.rng.gen_range(0..self.cards.len());
        Ok(self.cards.get(card_index).expect("Flashcard does not exist").clone())
    }
}
