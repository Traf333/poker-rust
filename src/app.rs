use iced::{Element, Task, Theme};

use crate::poker::{Game, Player};
use crate::ui::screens::{game_screen, home_screen};

pub fn run() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title(|_: &App| String::from("Poker"))
        .theme(App::theme)
        .run()
}

pub struct App {
    pub game: Game,
    pub screen: Screen,
    pub players: Vec<Player>,

}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    StartGame,
    BackToHome,
    GameCommand(Command),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Deal,
    Flop,
    Turn,
    River,
}

pub enum Screen {
    Home,
    Game,
}

impl Default for App {
    fn default() -> Self {

        App { game: Game::new(vec![]), screen: Screen::Home, players: Vec::new() }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StartGame => {
                self.players = vec![Player::new("Player 1".into()), Player::new("Player 2".into())];
                self.game = Game::new(self.players.clone());
                self.screen = Screen::Game;
                Task::none()
            }
            Message::GameCommand(Command::Deal) => {
                self.game.deal();
                Task::none()
            }
            Message::GameCommand(Command::Flop) => {
                self.game.flop();
                Task::none()
            }
            Message::GameCommand(Command::Turn) => {
                self.game.turn();
                Task::none()
            }
            Message::GameCommand(Command::River) => {
                self.game.river();
                Task::none()
            }
            Message::BackToHome => {
                self.screen = Screen::Home;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match self.screen {
            Screen::Home => home_screen::view(),
            Screen::Game => game_screen::view(&self.game),
        }
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }
}
