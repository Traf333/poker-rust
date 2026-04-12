use super::card::{Card, ACE, CLUBS, DIAMONDS, HEARTS, JACK, KING, QUEEN, SPADES};
use super::{Combination, Hand};
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
pub struct Player {
    pub hand: Hand,
    pub name: String,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player { hand: Hand::new([Card::new(0, 0), Card::new(0, 0)]), name }
    }
}

impl Game {
    pub fn new(players: Vec<Player>) -> Game {
        Game {
            players,
            deck: Deck::new(),
            community_cards: Vec::new(),
        }
    }

    pub fn winners(&self) -> Vec<(Combination, &Player)> {
        let mut potential_winners: Vec<(Combination, &Player)> = vec![];

        for player in self.players.iter() {
            let mut cards = Vec::with_capacity(7);
            cards.extend_from_slice(&player.hand.cards);
            cards.extend_from_slice(&self.community_cards);
            let combo = Combination::find_combination(&cards);
            if potential_winners.is_empty() {
                potential_winners.push((combo, player));
                continue;
            }
            if combo > potential_winners[0].0 {
                potential_winners.clear();
                potential_winners.push((combo, player));
            }
        };
        potential_winners
    }

    // function which deals the cards to player by one to every player and then deals for second card to every player
    pub fn deal(&mut self) {
        for i in 0..self.players.len() {
            self.players[i].hand.cards[0] = self.deck.cards.remove(0);
        }
        for i in 0..self.players.len() {
            self.players[i].hand.cards[1] = self.deck.cards.remove(0);
        }
    }

    pub fn flop(&mut self) {
        // remove 1 cover card
        self.deck.cards.remove(0);

        for i in 0..3 {
            self.community_cards.push(self.deck.cards.remove(0));
        }
    }

    pub fn turn(&mut self) {
                // remove 1 cover card
                self.deck.cards.remove(0);
        self.community_cards.push(self.deck.cards.remove(0));
    }
    
    pub fn river(&mut self) {
        // remove 1 cover card
        self.deck.cards.remove(0);
        self.community_cards.push(self.deck.cards.remove(0));
    }
}

impl Deck {
    pub fn  new() -> Deck {
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
        cards.shuffle(&mut rand::rng());
        Deck { cards }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal() {
        let mut game = Game::new(vec![Player::new("Player 1".into()), Player::new("Player 2".into())]);
        let deck_cards = game.deck.cards.clone();

        game.deal();

        assert_eq!(game.players[0].hand.cards[0], deck_cards[0]);
        assert_eq!(game.players[0].hand.cards[1], deck_cards[2]);
        assert_eq!(game.players[1].hand.cards[0], deck_cards[1]);
        assert_eq!(game.players[1].hand.cards[1], deck_cards[3]);
    }
}