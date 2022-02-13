use crate::{Command, Error, Player};

pub trait CommandReceiver {
    fn get_command(&self) -> Result<(Player, Command), Error>;
}
