#![allow(unused)]
use crate::FlashCard;

#[derive(Default)]
pub struct Config {
    pub attempts_before_wrong: usize,
    pub cards: Vec<FlashCard>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            attempts_before_wrong: 3,
            cards: vec![],
        }
    }

    pub fn build() -> Config {
        Config::new()
    }

    fn check_default_path() -> Option<()> {
        Some(())
    }
}

