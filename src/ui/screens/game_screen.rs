use iced::{
    widget::{column, container, row, text, Space},
    Alignment, Background, Border, Element, Fill,
};

use crate::poker::{Game, Player};
use crate::ui::components::card_view::card_chip;
use crate::ui::theme;

/// Root view of the game screen.
/// Renders the felt table, community cards, all player hands and the winner summary.
pub fn view<Message: Clone + 'static>(game: &Game) -> Element<'_, Message> {
    let winners = game.winners();
    let winner_names: Vec<&str> = winners.iter().map(|(_, p)| p.name.as_str()).collect();

    let title = text("♠  POKER  ♣")
        .size(42)
        .color(theme::GOLD);

    let community = community_section(game);
    let players = players_section(&game.players, &winner_names);
    let result = result_section(&winners);

    let content = column![
        title,
        Space::new().height(24),
        community,
        Space::new().height(32),
        players,
        Space::new().height(24),
        result,
    ]
    .align_x(Alignment::Center);

    container(content)
        .width(Fill)
        .height(Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(|_theme| container::Style {
            background: Some(Background::Color(theme::FELT)),
            ..container::Style::default()
        })
        .into()
}

fn community_section<Message: Clone + 'static>(game: &Game) -> Element<'static, Message> {
    let label = text("Community Cards")
        .size(15)
        .color(theme::LABEL);

    let cards = row(
        game.community_cards
            .iter()
            .map(|c| card_chip(*c))
            .collect::<Vec<_>>(),
    )
    .spacing(8);

    column![label, cards]
        .spacing(8)
        .align_x(Alignment::Center)
        .into()
}

fn players_section<'a, Message: Clone + 'static>(
    players: &'a [Player],
    winner_names: &[&str],
) -> Element<'a, Message> {
    let label = text("Players")
        .size(15)
        .color(theme::LABEL);

    let rows = column(
        players
            .iter()
            .map(|p| player_row(p, winner_names.contains(&p.name.as_str())))
            .collect::<Vec<_>>(),
    )
    .spacing(8);

    column![label, rows].spacing(10).into()
}

fn player_row<'a, Message: Clone + 'static>(
    player: &'a Player,
    is_winner: bool,
) -> Element<'a, Message> {
    let name_color = if is_winner { theme::GOLD } else { iced::Color::WHITE };

    let name = container(text(player.name.as_str()).size(16).color(name_color))
        .width(90);

    let cards = row(
        player.hand.cards.iter().map(|c| card_chip(*c)).collect::<Vec<_>>(),
    )
    .spacing(6);

    let inner = row![name, cards]
        .spacing(12)
        .align_y(Alignment::Center);

    if is_winner {
        container(inner)
            .padding([6, 14])
            .style(|_theme| container::Style {
                background: Some(Background::Color(theme::WINNER_TINT)),
                border: Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: theme::GOLD,
                },
                ..container::Style::default()
            })
            .into()
    } else {
        container(inner).padding([6, 14]).into()
    }
}

fn result_section<'a, Message: 'static>(
    winners: &[(crate::poker::Combination, &'a crate::poker::Player)],
) -> Element<'a, Message> {
    let lines = winners
        .iter()
        .map(|(combo, p)| format!("🏆  {}  —  {}", p.name, combo))
        .collect::<Vec<_>>()
        .join("\n");

    text(lines).size(20).color(theme::GOLD).into()
}
