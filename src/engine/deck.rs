use crate::engine::flashcard::FlashCard;
use log::debug;
use rand::prelude::*;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<FlashCard>,
    rng: ThreadRng
}

impl Deck {
    pub fn new(mut cards: Vec<FlashCard>) -> Result<Self, &'static str> {
        if cards.is_empty() {
            return Err("No FlashCards in config file");
        }
        cards.sort();
        
        Ok(Self { 
            cards,
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
