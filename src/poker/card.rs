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
            let (value, suite) = s.split_at(s.len() - 1);
            let v = match value {
                "A" => ACE,
                "K" => KING,
                "Q" => QUEEN,
                "J" => JACK,
                _ => value.parse::<u8>().unwrap(),
            };

            let s = match suite {
                "H" => HEARTS,
                "C" => CLUBS,
                "S" => SPADES,
                "D" => DIAMONDS,
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
        let cards = parse_cards("2H AD 10S 5C 6H");
        assert_eq!(cards.len(), 5);
        assert_eq!(cards[0].value, 2);
        assert_eq!(cards[0].suit, HEARTS);
        assert_eq!(cards[1].value, ACE);
        assert_eq!(cards[1].suit, DIAMONDS);
        assert_eq!(cards[2].suit, SPADES);
        assert_eq!(cards[2].value, 10);
        assert_eq!(cards[3].suit, CLUBS);
        assert_eq!(cards[3].value, 5);
        assert_eq!(cards[4].suit, HEARTS);
        assert_eq!(cards[4].value, 6);
    }
}
