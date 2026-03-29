use crate::poker::Card;

#[derive(Debug, Clone, PartialEq)]
pub struct Hand(Card, Card);

impl Hand {
    pub fn cards(&self) -> [Card; 2] {
        [self.0, self.1]
    }
}
