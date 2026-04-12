use iced::{Element, Task, Theme};

use crate::poker::{Game, Player};
use crate::ui::screens::game_screen;

pub fn run() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title(|_: &App| String::from("Poker"))
        .theme(App::theme)
        .run()
}

pub struct App {
    pub game: Game,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl Default for App {
    fn default() -> Self {
        let players = vec![
            Player::new("Alice".into()),
            Player::new("Bob".into()),
            Player::new("Charlie".into()),
            Player::new("Diana".into()),
        ];
        let mut game = Game::new(players);
        game.deal();
        game.flop();
        game.turn();
        game.river();
        App { game }
    }
}

impl App {
    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        game_screen::view(&self.game)
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }
}
