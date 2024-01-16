#![allow(unused)]
mod deck;
pub mod flashcard;
use std::error::Error;
use std::io::prelude::*;

pub fn run(config: crate::Config) -> Option<()> {
    let mut deck = deck::Deck::new(config.cards);
    deck.next_card().unwrap();
    Some(())
}

fn show_card(card: &crate::FlashCard, mut attempts: usize) -> Option<()> {
    Some(())
}

fn get_input() {}

