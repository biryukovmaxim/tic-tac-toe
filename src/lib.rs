use crate::command::Command;
use crate::command_receiver::CommandReceiver;
use crate::error::Error;
use crate::state::{Mark, Player, State};

pub mod command;
pub mod command_receiver;
pub mod error;
pub mod state;
pub mod stream_in;

const LENGTH: usize = 3;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum GameResult {
    Draw,
    Win(Player),
    InProgress,
}

pub struct Engine<T: CommandReceiver> {
    state: State,
    commands_receiver: T,
}

impl<T: CommandReceiver> Engine<T> {
    pub fn new(commands_receiver: T) -> Self {
        Self {
            state: Default::default(),
            commands_receiver,
        }
    }
}

impl<T: CommandReceiver> Engine<T> {
    pub fn run(mut self) -> GameResult {
        let mut player_cursor = Player::XPlayer;
        loop {
            match self
                .commands_receiver
                .get_command_from_player(player_cursor)
            {
                Err(e) => {
                    println!("{e}")
                }
                Ok(cmd) => match cmd {
                    Command::Turn(coordinate) => {
                        if let Err(e) = self.state.turn(player_cursor, coordinate).1 {
                            println!("wrong turn {e}")
                        } else {
                            let game_result = check_state(&self.state, player_cursor, coordinate);
                            match game_result {
                                GameResult::Draw => {
                                    println!("game over! Draw!");
                                    return game_result;
                                }
                                GameResult::Win(winner) => {
                                    println!("Congratulations! {winner:?} WIN!");
                                    return game_result;
                                }
                                GameResult::InProgress => player_cursor = player_cursor.inverse(),
                            }
                        }
                    }
                    Command::Surrender => {
                        let winner = player_cursor.inverse();
                        println!("Congratulations! {winner:?} WIN!");
                        return GameResult::Win(winner);
                    }
                },
            }
        }
    }
}

fn check_state(state: &State, p: Player, coordinate: usize) -> GameResult {
    let y = coordinate / LENGTH;
    let x = coordinate % LENGTH;
    let board = state.board();
    let predicate: Box<dyn Fn() -> Box<dyn FnOnce(Mark) -> bool>> = Box::new(|| match p {
        Player::XPlayer => Box::new(|mark: Mark| mark.is_x()),
        Player::OPlayer => Box::new(|mark: Mark| mark.is_o()),
    });

    let vert = (0..LENGTH)
        .map(|x| y * LENGTH + x)
        .all(|idx| predicate()(board[idx]));
    let horizontal = (0..LENGTH)
        .map(|y| y * LENGTH + x)
        .all(|idx| predicate()(board[idx]));
    let main_diagonal = (0..LENGTH)
        .map(|i| i * LENGTH + i)
        .all(|idx| predicate()(board[idx]));
    let secondary_diagonal = (0..LENGTH)
        .map(|i| (LENGTH - i - 1) * LENGTH + i)
        .all(|idx| predicate()(board[idx]));
    let all_filled = (0..LENGTH * LENGTH)
        .map(|idx| board[idx])
        .all(|m| !m.is_empty());

    if vert || horizontal || main_diagonal || secondary_diagonal {
        GameResult::Win(p)
    } else if all_filled {
        GameResult::Draw
    } else {
        GameResult::InProgress
    }
}
