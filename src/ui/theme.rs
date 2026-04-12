use iced::Color;

use crate::poker::{CLUBS, DIAMONDS, HEARTS, SPADES};

/// Deep casino felt green.
pub const FELT: Color = Color { r: 0.09, g: 0.30, b: 0.14, a: 1.0 };
/// Warm gold for highlights and winner callouts.
pub const GOLD: Color = Color { r: 1.0, g: 0.84, b: 0.0, a: 1.0 };
/// Subtle gold tint used as winner row background.
pub const WINNER_TINT: Color = Color { r: 0.9, g: 0.75, b: 0.0, a: 0.12 };
/// Muted green used for section labels.
pub const LABEL: Color = Color { r: 0.65, g: 0.88, b: 0.65, a: 1.0 };

pub fn suit_symbol(suit: u8) -> &'static str {
    match suit {
        SPADES => "♠",
        HEARTS => "♥",
        DIAMONDS => "♦",
        CLUBS => "♣",
        _ => "?",
    }
}

pub fn suit_color(suit: u8) -> Color {
    match suit {
        HEARTS | DIAMONDS => Color { r: 0.85, g: 0.12, b: 0.12, a: 1.0 },
        _ => Color { r: 0.10, g: 0.10, b: 0.10, a: 1.0 },
    }
}

pub fn card_value(value: u8) -> &'static str {
    match value {
        14 => "A",
        13 => "K",
        12 => "Q",
        11 => "J",
        10 => "10",
        9 => "9",
        8 => "8",
        7 => "7",
        6 => "6",
        5 => "5",
        4 => "4",
        3 => "3",
        2 => "2",
        _ => "?",
    }
}
