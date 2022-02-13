use crate::{Command, CommandReceiver, Error, Player};
use std::io;
use std::io::Read;

pub trait ReadLine: Read {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize>;
}

impl ReadLine for std::io::Stdin {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        std::io::Stdin::read_line(self, buf)
    }
}

pub struct StreamIn<T: ReadLine> {
    stream: T,
}

impl<T: ReadLine> StreamIn<T> {
    pub fn new(stream: T) -> Self {
        Self { stream }
    }
}

impl<T: ReadLine> CommandReceiver for StreamIn<T> {
    fn get_command_from_player(&mut self, player: Player) -> Result<Command, Error> {
        println!("игрок {player:?} введите команду:\n");
        let mut command: String = String::with_capacity(9);
        self.stream.read_line(&mut command)?;
        let command = Command::try_from(command.trim().to_lowercase().as_str())?;
        Ok(command)
    }
}
