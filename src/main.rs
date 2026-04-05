use poker::*;
use std::env;
use std::io::{self, Write};
use std::process;
use std::thread;
use std::time::Duration;

mod poker;

const ROUND_DELAY: Duration = Duration::from_millis(3);
const MIN_PLAYERS: usize = 2;
const MAX_PLAYERS: usize = 9;

fn parse_player_count() -> usize {
    match env::args().nth(1) {
        Some(s) => match s.parse::<usize>() {
            Ok(n) if (MIN_PLAYERS..=MAX_PLAYERS).contains(&n) => n,
            _ => {
                eprintln!(
                    "Error: players must be an integer between {MIN_PLAYERS} and {MAX_PLAYERS}."
                );
                usage();
            }
        },
        None => {
            eprintln!("Error: missing number of players.");
            usage();
        }
    }
}

fn usage() -> ! {
    let prog = env::args()
        .next()
        .unwrap_or_else(|| "pocker-rust".to_string());
    eprintln!(
        "Usage: {} <players>\n  players — how many players ({MIN_PLAYERS}–{MAX_PLAYERS})",
        prog
    );
    process::exit(1);
}

fn format_cards(cards: &[Card]) -> String {
    cards.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" ")
}

/// Clears the current terminal line and redraws `board` so flop/turn/river stay on one line.
fn print_board_same_line(board: &str) {
    print!("\r\x1b[2K{}", board);
    let _ = io::stdout().flush();
}

fn pause_between_rounds() {
    thread::sleep(ROUND_DELAY);
}

fn main() {
    let n = parse_player_count();
    let players: Vec<Player> = (1..=n)
        .map(|i| Player::new(format!("Player {i}")))
        .collect();
    let mut game = Game::new(players);

    game.deal();

    println!("Hole cards");
    let hole_line: String = game
        .players
        .iter()
        .map(|p| format!("{}: {}", p.name, p.hand))
        .collect::<Vec<_>>()
        .join("     ");
    println!("  {hole_line}");
    println!();

    pause_between_rounds();

    game.flop();
    print_board_same_line(&format!("Board: {}", format_cards(&game.community_cards)));

    pause_between_rounds();

    game.turn();
    print_board_same_line(&format!("Board: {}", format_cards(&game.community_cards)));

    pause_between_rounds();

    game.river();
    print_board_same_line(&format!("Board: {}", format_cards(&game.community_cards)));

    println!();

    pause_between_rounds();

    println!("Showdown");
    let winners = game.winners();
    for winner in winners {
        println!("{}: {}", winner.1.name, winner.0);
    }
}
