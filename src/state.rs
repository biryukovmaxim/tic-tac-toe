use crate::error::Error;

pub struct State {
    board: Board,
    turn: Player,
}

impl State {
    pub fn with_turn(p: Player) -> Self {
        Self {
            board: Default::default(),
            turn: p,
        }
    }

    fn switch_player_turn(&mut self) {
        self.turn = self.turn.inverse()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            board: Default::default(),
            turn: Player::XPlayer,
        }
    }
}

impl State {
    pub fn turn(&mut self, p: Player, coordinate: usize) -> Result<(), Error> {
        if p != self.turn {
            Err(Error::AnotherPlayerTurn(p, self.turn))
        } else {
            match self.board.0.get_mut(coordinate) {
                None => Err(Error::CoordinateNotExists(coordinate)),
                Some(m) => {
                    if let Mark::None = m {
                        match p {
                            Player::XPlayer => *m = Mark::X,
                            Player::OPlayer => *m = Mark::O,
                        }
                        self.switch_player_turn();
                        Ok(())
                    } else {
                        return Err(Error::CoordinateFilled(coordinate, *m));
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    XPlayer,
    OPlayer,
}

impl Player {
    pub fn inverse(self) -> Self {
        match self {
            Player::XPlayer => Player::OPlayer,
            Player::OPlayer => Player::XPlayer,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mark {
    None,
    O,
    X,
}

impl Default for Mark {
    fn default() -> Self {
        Mark::None
    }
}

#[derive(Default, Debug)]
struct Board([Mark; 9]);

#[cfg(test)]
mod tests {
    use crate::state::*;

    #[test]
    fn turns() {
        let mut state = State::default();
        let bad_coordinate = state.turn(Player::XPlayer, 10);
        assert_eq!(bad_coordinate, Err(Error::CoordinateNotExists(10)));

        let first_turn = state.turn(Player::XPlayer, 0);
        assert_eq!(first_turn, Ok(()));

        let double_turn = state.turn(Player::XPlayer, 0);
        assert_eq!(
            double_turn,
            Err(Error::AnotherPlayerTurn(Player::XPlayer, Player::OPlayer))
        );

        let filled_turn = state.turn(Player::OPlayer, 0);
        assert_eq!(filled_turn, Err(Error::CoordinateFilled(0, Mark::X)));
    }
}
