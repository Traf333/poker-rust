#![allow(dead_code, unused)]
use poker::*;

mod poker;
fn main() {
    let players = vec![];
    let game = Game::new(players);

    // game.deck.cards print cards with spaces
    for card in game.deck.cards {
        print!("{} ", card);
    }
}
