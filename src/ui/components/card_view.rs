use iced::{
    widget::{container, text},
    Background, Border, Color, Element,
};

use crate::poker::Card;
use crate::ui::theme;

/// Renders a single playing card as a small white chip with coloured suit symbol.
/// Takes `Card` by value — `Card` is `Copy` so callers pass `*card`.
pub fn card_chip<Message: 'static>(card: Card) -> Element<'static, Message> {
    let label = format!(
        "{}{}",
        theme::card_value(card.value),
        theme::suit_symbol(card.suit)
    );
    let color = theme::suit_color(card.suit);

    container(text(label).size(22).color(color))
        .padding([6, 10])
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::WHITE)),
            border: Border {
                radius: 6.0.into(),
                width: 1.5,
                color: Color { r: 0.75, g: 0.75, b: 0.75, a: 1.0 },
            },
            ..container::Style::default()
        })
        .into()
}
