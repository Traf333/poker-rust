use crate::poker::{ACE, CLUBS, Card, DIAMONDS, HEARTS, JACK, KING, QUEEN, SPADES};
use std::fmt::Display;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Combination {
    HighCard([u8; 5]),
    Pair(u8, [u8; 3]),
    TwoPair(u8, u8, u8),
    ThreeOfKind(u8, [u8; 2]),
    Straight(u8),
    Flush([u8; 5]),
    FullHouse(u8, u8),
    FourOfKind(u8, u8),
    StraightFlush(u8),
}

fn find_straight(sorted: &[Card]) -> Option<u8> {
    let mut ranks: Vec<u8> = sorted.iter().map(|card| card.value).collect();

    ranks.dedup();

    for rank in ranks.windows(5) {
        if rank[0] - rank[4] == 4 {
            return Some(rank[0]);
        }
    }

    let has_wheel = ranks.contains(&ACE) && [2, 3, 4, 5].iter().all(|&v| ranks.contains(&v));
    if has_wheel { Some(5) } else { None }
}

fn find_flush(sorted: &[Card]) -> Option<[u8; 5]> {
    for suit in [HEARTS, SPADES, DIAMONDS, CLUBS] {
        let suited: Vec<u8> = sorted
            .iter()
            .filter(|card| card.suit == suit)
            .map(|c| c.value)
            .collect();
        if suited.len() >= 5 {
            return Some([suited[0], suited[1], suited[2], suited[3], suited[4]]);
        }
    }
    None
}

fn find_straight_flush(sorted: &[Card]) -> Option<u8> {
    for suit in [HEARTS, SPADES, DIAMONDS, CLUBS] {
        let suited: Vec<Card> = sorted
            .iter()
            .filter(|card| card.suit == suit)
            .copied()
            .collect();

        if suited.len() >= 5 {
            if let Some(top) = find_straight(&suited) {
                return Some(top);
            }
        }
    }
    None
}

impl Combination {
    pub fn find_combination(cards: &[Card]) -> Self {
        assert!(cards.len() >= 5, "Need at least 5 cards");

        let mut sorted = cards.to_vec();
        sorted.sort_unstable_by(|a, b| b.value.cmp(&a.value));

        let mut groups: Vec<(u8, u8)> = Vec::new();
        for card in &sorted {
            match groups.iter_mut().find(|(rank, _)| *rank == card.value) {
                Some(group) => group.1 += 1,
                None => groups.push((card.value, 1)),
            }
        }
        groups.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));

        let is_flush = find_flush(&sorted);
        let is_straight = find_straight(&sorted);
        let is_straight_flush = find_straight_flush(&sorted);

        if let Some(top_rank) = is_straight_flush {
            return Combination::StraightFlush(top_rank);
        }

        match groups.as_slice() {
            [(quad, 4), (kicker, _), ..] => Combination::FourOfKind(*quad, *kicker),
            [(trip, 3), (pair, 2), ..] => Combination::FullHouse(*trip, *pair),
            [(trip, 3), (low_trip, 3), ..] => Combination::FullHouse(*trip, *low_trip),
            _ if is_flush.is_some() => Combination::Flush(is_flush.unwrap()),
            _ if is_straight.is_some() => Combination::Straight(is_straight.unwrap()),
            [(three, 3), (k1, 1), (k2, 1), ..] => Combination::ThreeOfKind(*three, [*k1, *k2]),
            [(high_pair, 2), (low_pair, 2), (k, 1), ..] => {
                Combination::TwoPair(*high_pair, *low_pair, *k)
            }
            [(pair, 2), (k1, 1), (k2, 1), (k3, 1), ..] => Combination::Pair(*pair, [*k1, *k2, *k3]),
            [(k1, 1), (k2, 1), (k3, 1), (k4, 1), (k5, 1), ..] => {
                Combination::HighCard([*k1, *k2, *k3, *k4, *k5])
            }
            _ => unreachable!("Invalid hand"),
        }
    }
}

fn format_rank(r: u8) -> String {
    match r {
        ACE => "A".to_string(),
        KING => "K".to_string(),
        QUEEN => "Q".to_string(),
        JACK => "J".to_string(),
        n => n.to_string(),
    }
}

fn format_ranks(cards: &[u8]) -> String {
    cards.iter().map(|&r| format_rank(r)).collect::<Vec<_>>().join(" ")
}

impl Display for Combination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Combination::StraightFlush(top_rank) => {
                write!(f, "Straight Flush of {}", format_rank(*top_rank))
            }
            Combination::FourOfKind(quad, kicker) => write!(
                f,
                "Four of a Kind of {} with {}",
                format_rank(*quad),
                format_rank(*kicker)
            ),
            Combination::FullHouse(trip, pair) => write!(
                f,
                "Full House of {} and {}",
                format_rank(*trip),
                format_rank(*pair)
            ),
            Combination::Flush(cards) => write!(f, "Flush of {}", format_ranks(cards)),
            Combination::Straight(top_rank) => write!(f, "Straight of {}", format_rank(*top_rank)),
            Combination::ThreeOfKind(trip, kicker) => write!(
                f,
                "Three of a Kind of {} with {}",
                format_rank(*trip),
                format_ranks(kicker)
            ),
            Combination::TwoPair(high_pair, low_pair, kicker) => write!(
                f,
                "Two Pair of {} and {} with {}",
                format_rank(*high_pair),
                format_rank(*low_pair),
                format_rank(*kicker)
            ),
            Combination::Pair(pair, kicker) => write!(
                f,
                "Pair of {} with {}",
                format_rank(*pair),
                format_ranks(kicker)
            ),
            Combination::HighCard(cards) => write!(f, "High Card of {}", format_ranks(cards)),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::poker::card::parse_cards;

    use super::*;

    #[test]
    fn test_royal_flush() {
        let cards = parse_cards("JH AH KH QH 10H 9H 8H");

        assert_eq!(
            Combination::find_combination(&cards),
            Combination::StraightFlush(ACE)
        );
    }

    #[test]
    fn test_straight_flush() {
        let cards = parse_cards("JH AD KH QH 10H 9H 8H");
        assert_eq!(
            Combination::find_combination(&cards),
            Combination::StraightFlush(KING)
        );
    }

    #[test]
    fn test_straight_flush_with_king() {
        let cards = parse_cards("AH KH 2H 3H 4H 5H QH");
        assert_eq!(
            Combination::find_combination(&cards),
            Combination::StraightFlush(5)
        );
    }

    #[test]
    fn test_straight_flush_with_ace() {
        let cards = parse_cards("2H 3H 4H AH 5H 10D 7C");
        assert_eq!(
            Combination::find_combination(&cards),
            Combination::StraightFlush(5)
        );
    }
    #[test]
    fn test_four_of_kind() {
        let cards = parse_cards("JH JD JS JC 10H 9H 8H");
        assert_eq!(
            Combination::find_combination(&cards),
            Combination::FourOfKind(JACK, 10)
        );
    }
    #[test]
    fn test_full_house() {
        let cards = parse_cards("JH JD JS QH QD 9H 8H");
        assert_eq!(
            Combination::find_combination(&cards),
            Combination::FullHouse(JACK, QUEEN)
        );
    }

    #[test]
    fn test_full_house_with_two_sets() {
        let cards = parse_cards("JH JD JS QH QD QD 8H");
        assert_eq!(
            Combination::find_combination(&cards),
            Combination::FullHouse(QUEEN, JACK)
        );
    }
    #[test]
    fn test_flush() {
        let cards = parse_cards("JH JD JS QH 2H 9H 8H");
        assert_eq!(
            Combination::find_combination(&cards),
            Combination::Flush([QUEEN, JACK, 9, 8, 2])
        );
    }
    #[test]
    fn test_straight() {
        let cards = parse_cards("JH JD KH QC 10H 9S 8H");
        assert_eq!(
            Combination::find_combination(&cards),
            Combination::Straight(KING)
        );
    }

    #[test]
    fn test_straight_with_ace() {
        let cards = parse_cards("AH 2H 3S 4H 5H 10D 7C");
        assert_eq!(
            Combination::find_combination(&cards),
            Combination::Straight(5)
        );
    }
    #[test]
    fn test_three_of_kind() {
        let cards = parse_cards("JH JD KH QS 10H JS 8H");
        assert_eq!(
            Combination::find_combination(&cards),
            Combination::ThreeOfKind(JACK, [KING, QUEEN])
        );
    }
    #[test]
    fn test_two_pair() {
        let cards = parse_cards("JH JD QC QH 10C 9H 7H");
        assert_eq!(
            Combination::find_combination(&cards),
            Combination::TwoPair(QUEEN, JACK, 10)
        );
    }
    #[test]
    fn test_pair() {
        let cards = parse_cards("JH JD KD 2H 10H 9H AC");
        assert_eq!(
            Combination::find_combination(&cards),
            Combination::Pair(JACK, [ACE, KING, 10])
        );
    }
    #[test]
    fn test_high_card() {
        let cards = parse_cards("AH JD 5D QH 10H 9D 4H");
        assert_eq!(
            Combination::find_combination(&cards),
            Combination::HighCard([ACE, QUEEN, JACK, 10, 9])
        );
    }
}
