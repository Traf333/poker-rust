use crate::poker::{ACE, CLUBS, Card, DIAMONDS, HEARTS, JACK, KING, QUEEN, SPADES};

use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

#[derive(Debug, PartialEq)]
pub enum Combination {
    RoyalFlush,
    StraightFlush,
    FourOfKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfKind,
    TwoPair,
    Pair,
    HighCard,
}

fn check_straight(cards: &[Card]) -> bool {
    let mut uniq_reverse_numbers = cards.iter().map(|card| card.value).collect::<Vec<u8>>();

    uniq_reverse_numbers.dedup();

    if uniq_reverse_numbers.contains(&ACE) {
        uniq_reverse_numbers.push(1);
    }

    if uniq_reverse_numbers.len() < 5 {
        return false;
    }

    uniq_reverse_numbers.sort_by(|a, b| b.cmp(a));

    for i in 0..uniq_reverse_numbers.len() - 4 {
        if uniq_reverse_numbers[i..i + 5]
            .windows(2)
            .all(|window| window[0] - window[1] == 1)
        {
            return true;
        }
    }
    false
}

impl Combination {
    pub fn find(cards: &[Card]) -> Self {
        let mut is_flush = false;
        let mut cards = cards.to_vec();
        cards.sort_by_key(|card| card.value);
        for suit in [HEARTS, SPADES, DIAMONDS, CLUBS] {
            let mut same_suit_cards = cards
                .iter()
                .filter(|card| card.suit == suit)
                .cloned()
                .collect::<Vec<Card>>();

            if same_suit_cards.len() < 5 {
                continue;
            }
            is_flush = true;

            if check_straight(&same_suit_cards) {
                if [ACE, KING, JACK, QUEEN].iter().all(|value| {
                    same_suit_cards.contains(&Card { value: *value, suit })
                })
                {
                    return Self::RoyalFlush;
                }
                return Self::StraightFlush;
            }
        }
        let is_straight = check_straight(&cards);

        let mut map = HashMap::new();
        for card in cards.iter() {
            *map.entry(card.value).or_insert(0) += 1;
        }

        let values = map.values().copied().collect::<Vec<u8>>();
        if values.contains(&4) {
            return Self::FourOfKind;
        }
        if (values.contains(&3) && values.contains(&2))  || values.iter().filter(|v| **v == 3).count() == 2{
            return Self::FullHouse;
        }
        if is_flush {
            return Self::Flush;
        }
        if is_straight {
            return Self::Straight;
        }
        if values.contains(&3) {
            return Self::ThreeOfKind;
        }
        if values.iter().filter(|v| **v == 2).count() == 2 {
            return Self::TwoPair;
        }

        if values.contains(&2) {
            return Self::Pair;
        }

        Self::HighCard
    }
}
#[cfg(test)]
mod tests {
    use crate::poker::card::parse_cards;

    use super::*;

    #[test]
    fn test_royal_flush() {
        let cards = parse_cards("JH AH KH QH 10H 9H 8H");

        assert_eq!(Combination::find(&cards), Combination::RoyalFlush);
    }

    #[test]
    fn test_straight_flush() {
        let cards = parse_cards("JH AD KH QH 10H 9H 8H");
        assert_eq!(Combination::find(&cards), Combination::StraightFlush);
    }

    #[test]
    fn test_straight_flush_with_king() {
        let cards = parse_cards("AH KH 2H 3H 4H 5H QH");
        assert_eq!(Combination::find(&cards), Combination::StraightFlush);
    }

    #[test]
    fn test_straight_flush_with_ace() {
        let cards = parse_cards("2H 3H 4H AH 5H 10D 7C");
        assert_eq!(Combination::find(&cards), Combination::StraightFlush);
    }
    #[test]
    fn test_four_of_kind() {
        let cards = parse_cards("JH JD JS JC 10H 9H 8H");
        assert_eq!(Combination::find(&cards), Combination::FourOfKind);
    }
    #[test]
    fn test_full_house() {
        let cards = parse_cards("JH JD JS QH QD 9H 8H");
        assert_eq!(Combination::find(&cards), Combination::FullHouse);
    }

    #[test]
    fn test_full_house_with_two_sets() {
        let cards = parse_cards("JH JD JS QH QD QD 8H");
        assert_eq!(Combination::find(&cards), Combination::FullHouse);
    }
    #[test]
    fn test_flush() {
        let cards = parse_cards("JH JD JS QH 2H 9H 8H");
        assert_eq!(Combination::find(&cards), Combination::Flush);
    }
    #[test]
    fn test_straight() {
        let cards = parse_cards("JH JD KH QC 10H 9S 8H");
        assert_eq!(Combination::find(&cards), Combination::Straight);
    }

    #[test]
    fn test_straight_with_ace() {
        let cards = parse_cards("AH 2H 3S 4H 5H 10D 7C");
        assert_eq!(Combination::find(&cards), Combination::Straight);
    }
    #[test]
    fn test_three_of_kind() {
        let cards = parse_cards("JH JD KH QS 10H JS 8H");
        assert_eq!(Combination::find(&cards), Combination::ThreeOfKind);
    }
    #[test]
    fn test_two_pair() {
        let cards = parse_cards("JH JD QC QH 10C 9H 7H");
        assert_eq!(Combination::find(&cards), Combination::TwoPair);
    }
    #[test]
    fn test_pair() {
        let cards = parse_cards("JH JD KD 2H 10H 9H AC");
        assert_eq!(Combination::find(&cards), Combination::Pair);
    }
    #[test]
    fn test_high_card() {
        let cards = parse_cards("AH JD 5D QH 10H 9D 4H");
        assert_eq!(Combination::find(&cards), Combination::HighCard);
    }
}
