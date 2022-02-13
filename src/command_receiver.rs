use crate::{Command, Error, Player};

pub trait CommandReceiver {
    fn get_command_from_player(&mut self, player: Player) -> Result<Command, Error>;
}
