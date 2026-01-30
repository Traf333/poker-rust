use crate::poker::{ACE, CLUBS, Card, DIAMONDS, HEARTS, JACK, KING, QUEEN, SPADES};

use std::{collections::HashMap, iter::FromIterator};

#[derive(Debug, PartialEq)]
pub enum Combination {
    RoyalFlesh,
    StraightFlush,
    FourOfKind,
    FullHouse,
    Flash,
    Straight,
    ThreeOfKind,
    TwoPair,
    Pair,
    HighCard,
}

impl Combination {
    pub fn find(cards: &[Card; 7]) -> Self {
        for suit in [HEARTS, SPADES, DIAMONDS, CLUBS] {
            let mut same_suit_cards = cards
                .iter()
                .filter(|card| card.suit == suit)
                .cloned()
                .collect::<Vec<Card>>();

            // if cards include ace shift 1 at the beginnig
            if same_suit_cards.contains(&Card { value: ACE, suit }) {
                same_suit_cards.insert(0, Card { value: 1, suit });
            }
            same_suit_cards.sort_by_key(|card| card.value);

            let is_flash = same_suit_cards.len() >= 5;

            let mut is_straight = false;
            for i in (0..3).rev() {
                is_straight = same_suit_cards[i..i + 5]
                    .windows(2)
                    .all(|window| window[1].value == window[0].value + 1);
                if is_straight {
                    break;
                }
            }

            if is_flash && is_straight {
                if same_suit_cards.contains(&Card { value: ACE, suit }) {
                    return Self::RoyalFlesh;
                }
                return Self::StraightFlush;
            }
        }

        let mut map = HashMap::new();
        for card in cards.iter() {
            *map.entry(card.value).or_insert(0) += 1;
        }

        let mut values = map.values().copied().collect::<Vec<u8>>();
        values.sort();
        match values.as_slice() {
            [.., 4] => Self::FourOfKind,
            [.., 2, 3] => Self::FullHouse,
            [.., 3] => Self::ThreeOfKind,
            [.., 2, 2] => Self::TwoPair,
            [.., 2] => Self::Pair,
            _ => Self::HighCard,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_royal_flush() {
        let cards = [
            Card {
                value: ACE,
                suit: HEARTS,
            },
            Card {
                value: KING,
                suit: HEARTS,
            },
            Card {
                value: QUEEN,
                suit: HEARTS,
            },
            Card {
                value: JACK,
                suit: HEARTS,
            },
            Card {
                value: 10,
                suit: HEARTS,
            },
            Card {
                value: 9,
                suit: HEARTS,
            },
            Card {
                value: 8,
                suit: HEARTS,
            },
        ];

        assert_eq!(Combination::find(&cards), Combination::RoyalFlesh);
    }
}
