use iced::{widget::{button, column, container}, Alignment, Background, Element, Fill};
use iced::widget::text;
use crate::ui::theme;
use crate::app::Message;

pub fn view() -> Element<'static, Message> {
    let start_button = button(text("Start Game")).on_press(Message::StartGame);
    let content = column![
        text("Welcome to Poker"),
        start_button,
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