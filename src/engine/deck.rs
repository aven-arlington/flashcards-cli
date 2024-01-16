use crate::FlashCard;

#[derive(Default)]
pub struct Deck {
    cards: Vec<FlashCard>,
}

impl Deck {
    pub fn new(cards_from_config: Vec<FlashCard>) -> Self {
        Self {
            cards: cards_from_config,
        }
    }

    pub fn next_card(&mut self) -> Option<()> {
        Some(())
    }

    pub fn draw_hand(&mut self) {}

    pub fn hand_count(&self) -> Option<()> {
        Some(())
    }

    pub fn increase_level_pool(&mut self) {}
}

