use crate::Card;
use crate::poker::{Combination, Hand};
use crate::{ACE, CLUBS, DIAMONDS, HEARTS, JACK, KING, QUEEN, SPADES};
use rand::rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub deck: Deck,
    pub community_cards: Vec<Card>,
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

#[derive(Debug)]
pub struct Player(Hand);

impl Game {
    pub fn new(players: Vec<Player>) -> Game {
        Game {
            players,
            deck: Deck::new(),
            community_cards: Vec::new(),
        }
    }

    pub fn winners<'a>(&self, hands: &'a [Hand]) -> Vec<&'a Hand> {
        let mut potential_winners: Vec<&Hand> = vec![];

        for hand in hands {
            if (potential_winners.is_empty()) {
                potential_winners.push(hand);
                continue;
            }
        }

        potential_winners
    }
}

impl Deck {
    pub fn new() -> Deck {
        let values = vec![ACE, 2, 3, 4, 5, 6, 7, 8, 9, 10, JACK, QUEEN, KING];
        let suits = vec![SPADES, HEARTS, DIAMONDS, CLUBS];

        // shuffle cards
        let mut cards = Vec::new();
        for suit in suits {
            for value in values.iter() {
                cards.push(Card {
                    value: *value,
                    suit,
                });
            }
        }
        // cards.shuffle(&mut rand::rng());
        Deck { cards }
    }
}
