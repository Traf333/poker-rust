use std::fmt::Display;

pub const SPADES: u8 = 0;
pub const HEARTS: u8 = 1;
pub const DIAMONDS: u8 = 2;
pub const CLUBS: u8 = 3;

pub const ACE: u8 = 1;
pub const KING: u8 = 13;
pub const QUEEN: u8 = 12;
pub const JACK: u8 = 11;

#[derive(Debug)]
pub struct Card {
    pub value: u8,
    pub suit: u8,
}

impl Card {
    pub fn new(value: u8, suit: u8) -> Card {
        Card { value, suit }
    }
}

#[derive(Debug)]
pub struct Hand(Card, Card);

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suite = match self.suit {
            SPADES => "S",
            HEARTS => "H",
            DIAMONDS => "D",
            CLUBS => "C",
            _ => "Unknown",
        };

        let value = match self.value {
            ACE => "A",
            KING => "K",
            QUEEN => "Q",
            JACK => "J",
            _ => &self.value.to_string(),
        };
        write!(f, "{}{}", value, suite)
    }
}
