use crate::engine::flashcard::FlashCard;
use std::ops::Range;
use log::debug;
use rand::prelude::*;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<FlashCard>,
    level_indexes: Vec<usize>,
    level_range: Range<usize>,
    rng: ThreadRng
}

impl Deck {
    pub fn new(mut cards: Vec<FlashCard>) -> Result<Self, &'static str> {
        if cards.is_empty() {
            return Err("No FlashCards in config file");
        }
        cards.sort();
        
        let mut level_indexes = Vec::new();
        for (i, card) in cards.iter().enumerate() {
            if level_indexes.is_empty() {
                level_indexes.push(i);
                continue;
            }
            if card.level > cards[level_indexes[level_indexes.len()-1]].level {
                level_indexes.push(i);
                continue;
            }
        }

        let level_range = Range { start: 0, end: 1 };


        Ok(Self { 
            cards,
            level_indexes,
            level_range,
            rng:rand::thread_rng(),
        })
    }

    pub fn next(&self) -> Result<FlashCard, &'static str> {
        Ok(self.cards.get(0).expect("No Flashcards in deck").clone())
    }

    pub fn print_cards(&self) {
        for (i, card) in self.cards.iter().enumerate() {
            debug!("Deck position {} - {}", i, card.clue_side);
        }
        debug!("Level Stratifications:");
        for index in &self.level_indexes {
            debug!("Deck index {}", index);
        }
    }

    pub fn random_card(&mut self) -> Result<FlashCard, &'static str> {
        let card_index = self.rng.gen_range(0..self.cards.len());
        Ok(self.cards.get(card_index).expect("Flashcard does not exist").clone())
    }

    pub fn increase_level_max(&mut self) {
        if self.level_range.end >= self.level_indexes.len() -1 {
            println!("The maximum level has been reached");
            return;
        }
        println!("The maximum difficutly has increased!");
        self.level_range.end += 1;
    }

    pub fn increase_level_min(&mut self) {
        if self.level_range.start > self.level_range.end {
            println!("The minimum level cannot be higher than the maximum.");
            return;
        }
        if self.level_range.start >= self.level_indexes.len() -1 {
            println!("The highest minimum level has been reached");
            return;
        }
        println!("The minimum difficutly has increased!");
        self.level_range.start += 1;
    }

    pub fn reset_level(&mut self) {
        self.level_range = Range{ start: 0, end: 0 };
    }

    pub fn random_card_from_current_level_range(&mut self) -> Result<FlashCard, &'static str> {
        let card_index = self.rng.gen_range(self.level_indexes[self.level_range.start]..self.level_indexes[self.level_range.end]);
        Ok(self.cards.get(card_index).expect("Flashcard does not exist").clone())
    }
}
