use crate::state::{Mark, Player};
use std::io;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("it's not `{0:?}`'s turn to go. `{1:?}` should turn")]
    AnotherPlayerTurn(Player, Player),
    #[error("coordinate `{0:?}` is already filled by `{1:?}`")]
    CoordinateFilled(usize, Mark),
    #[error("coordinate `{0:?}` does not exists on board")]
    CoordinateNotExists(usize),
    #[error("read command error:`{0:?}`")]
    ReadCommandError(String),
    #[error("unknown command: `{0:?}`")]
    ParseCommandError(String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::ReadCommandError(e.to_string())
    }
}
