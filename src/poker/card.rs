use std::fmt::Display;

pub const SPADES: u8 = 0;
pub const HEARTS: u8 = 1;
pub const DIAMONDS: u8 = 2;
pub const CLUBS: u8 = 3;

pub const ACE: u8 = 14;
pub const KING: u8 = 13;
pub const QUEEN: u8 = 12;
pub const JACK: u8 = 11;

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub value: u8,
    pub suit: u8,
}

impl Card {
    pub fn new(value: u8, suit: u8) -> Card {
        Card { value, suit }
    }
}

pub fn parse_cards(str: &str) -> Vec<Card> {
    str.split_whitespace()
        .map(|s| {
            let card_chars = s.chars().into_iter().collect::<Vec<char>>();
            let [value, suit, ..] = card_chars.as_slice() else {
                panic!("Invalid card");
            };
            let v = match value {
                'A' => ACE,
                'K' => KING,
                'Q' => QUEEN,
                'J' => JACK,
                _ => value.to_digit(10).unwrap() as u8,
            };

            let s = match suit {
                'H' => HEARTS,
                'C' => CLUBS,
                'S' => SPADES,
                'D' => DIAMONDS,
                _ => panic!("Invalid suit"),
            };
            Card::new(v, s)
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cards() {
        let cards = parse_cards("2H AD 4S 5C 6H");
        assert_eq!(cards.len(), 5);
        assert_eq!(cards[0].value, 2);
        assert_eq!(cards[0].suit, HEARTS);
        assert_eq!(cards[1].value, ACE);
        assert_eq!(cards[1].suit, DIAMONDS);
    }
}
