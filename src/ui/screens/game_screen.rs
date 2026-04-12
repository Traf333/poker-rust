use iced::{
    widget::{button, column, container, row, text, Space},
    Alignment, Background, Border, Element, Fill,
};

use crate::poker::{Card, Game, Player};
use crate::ui::components::card_view::card_chip;
use crate::ui::theme;

use crate::app::{Command, Message};

/// Root view of the game screen.
/// Renders the felt table, community cards, all player hands and the winner summary.
pub fn view<'a>(game: &'a Game) -> Element<'a, Message> {
    let go_back_button = button(text("Go Back")).on_press(Message::BackToHome);
    let new_game_button = button(text("New Game")).on_press(Message::StartGame);
    let flop_button = button(text("Flop")).on_press(Message::GameCommand(Command::Flop));
    let deal_button = button(text("Deal")).on_press(Message::GameCommand(Command::Deal));
    let turn_button = button(text("Turn")).on_press(Message::GameCommand(Command::Turn));
    let river_button = button(text("River")).on_press(Message::GameCommand(Command::River));

    let buttons = row![deal_button, flop_button, turn_button, river_button]
        .spacing(8);

    // show winners only if all cards are dealt
    let winners = if game.community_cards.len() == 5 {
        game.winners()
    } else {
        vec![]
    };
    let winner_names: Vec<&str> = winners.iter().map(|(_, p)| p.name.as_str()).collect();

    let title = text("♠  POKER  ♣")
        .size(42)
        .color(theme::GOLD);

    let community = community_section(&game.community_cards);
    let players = players_section(&game.players, &winner_names);
    let result = result_section(&winners);

    let content = column![
        row![go_back_button, new_game_button],
        title,
        buttons,
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

fn community_section<Message: Clone + 'static>(community_cards: &Vec<Card>) -> Element<'static, Message> {
    let label = text("Community Cards")
        .size(15)
        .color(theme::LABEL);

    let cards = row(
        community_cards
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
