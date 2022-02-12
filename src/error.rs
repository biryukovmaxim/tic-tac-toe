use crate::state::{Mark, Player};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("it's not `{0:?}`'s turn to go. `{1:?}` should turn")]
    AnotherPlayerTurn(Player, Player),
    #[error("coordinate `{0:?}` is already filled by `{1:?}`")]
    CoordinateFilled(usize, Mark),
    #[error("coordinate `{0:?}` does not exists on board")]
    CoordinateNotExists(usize),
}
