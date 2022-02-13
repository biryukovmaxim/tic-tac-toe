use engine::command::Command;
use engine::command_receiver::CommandReceiver;
use engine::error::Error;
use engine::state::Player;
use engine::Engine;
use std::cell::RefCell;
use std::io;
use std::io::Read;

fn main() {
    let engine = Engine::new(StreamIn::new(std::io::stdin()));
    engine.run();
}

pub trait ReadLine: Read {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize>;
}

impl ReadLine for std::io::Stdin {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        std::io::Stdin::read_line(self, buf)
    }
}

pub struct StreamIn<T: ReadLine> {
    stream: RefCell<T>,
    player: RefCell<Player>,
}

impl<T: ReadLine> StreamIn<T> {
    pub fn new(stream: T) -> Self {
        Self {
            stream: RefCell::new(stream),
            player: RefCell::new(Player::XPlayer),
        }
    }
}

impl<T: ReadLine> CommandReceiver for StreamIn<T> {
    fn get_command(&self) -> Result<(Player, Command), Error> {
        let mut stream = self.stream.borrow_mut();
        let player = *self.player.borrow();
        println!("игрок {player:?} введите команду:\n");
        let mut command: String = String::with_capacity(9);
        stream.read_line(&mut command)?;
        let command = Command::try_from(command.trim().to_lowercase().as_str())?;
        *self.player.borrow_mut() = player.inverse();
        Ok((player, command))
    }
}
