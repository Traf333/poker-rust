use crate::poker::Card;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Hand {
    pub cards: [Card; 2],
}

impl Hand {
    pub fn new(cards: [Card; 2]) -> Hand {
        Hand { cards }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.cards[0], self.cards[1])
    }
}