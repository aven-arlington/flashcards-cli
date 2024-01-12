use crate::FlashCard;
use core::cmp::min;
use rand::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Deck {
    cards: BTreeMap<u32, Vec<FlashCard>>,
    hand: Vec<FlashCard>,
    available_levels: Vec<u32>,
    current_levels: Vec<u32>,
    rng: ThreadRng,
}

impl Deck {
    pub fn new(mut cards_from_config: Vec<FlashCard>) -> Deck {
        cards_from_config.sort();
        let mut cards: BTreeMap<u32, Vec<FlashCard>> = BTreeMap::new();
        for card in cards_from_config {
            let key = card.level.unwrap_or_default();
            cards
                .entry(key)
                .and_modify(|v| v.push(card.clone()))
                .or_insert(vec![card.clone()]);
        }

        let available_levels: Vec<u32> = cards.keys().cloned().collect();
        let current_levels: Vec<u32> = vec![available_levels[0]];
        let hand = Vec::new();

        Self {
            cards,
            hand,
            available_levels,
            current_levels,
            rng: rand::thread_rng(),
        }
    }

    pub fn next_card(&mut self) -> Option<FlashCard> {
        self.hand.pop()
    }

    pub fn draw_hand(&mut self) {
        let mut cards_to_draw_from: Vec<FlashCard> = Vec::new();
        for level in &self.current_levels {
            cards_to_draw_from.append(&mut self.cards.get(level).unwrap().clone());
        }
        cards_to_draw_from.shuffle(&mut self.rng);

        self.hand.clear();
        let hand_size = min(cards_to_draw_from.len(), 5);

        self.hand.resize(hand_size, FlashCard::default());
        self.hand
            .clone_from_slice(&cards_to_draw_from[0..hand_size]);
    }

    pub fn hand_count(&self) -> usize {
        self.hand.len()
    }

    pub fn increase_level_pool (&mut self) {
        for level in &self.available_levels {
            if self.current_levels.contains(level) {
                continue;
            }
            println!("Adding level {} to the deck", level);
            self.current_levels.push(*level);
            break;
        }
    }
}

