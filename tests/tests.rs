use engine::command::Command;
use engine::command_receiver::CommandReceiver;
use engine::error::Error;
use engine::state::Player;
use engine::state::Player::{OPlayer, XPlayer};
use engine::{Engine, GameResult};

#[test]
fn winners() {
    assert_eq!(run_test(MockCmdReceiver::x_win()), GameResult::Win(XPlayer));
    assert_eq!(run_test(MockCmdReceiver::o_win()), GameResult::Win(OPlayer));
}
#[test]
fn surrender() {
    assert_eq!(
        run_test(MockCmdReceiver::x_surrender()),
        GameResult::Win(OPlayer)
    );
    assert_eq!(
        run_test(MockCmdReceiver::o_surrender()),
        GameResult::Win(XPlayer)
    );
}
#[test]
fn draw() {
    assert_eq!(run_test(MockCmdReceiver::draw()), GameResult::Draw);
}

fn run_test(cmd_receiver: MockCmdReceiver) -> GameResult {
    let engine = Engine::new(cmd_receiver);
    engine.run()
}

pub struct MockCmdReceiver {
    commands: Vec<&'static str>,
    cursor: usize,
}

impl CommandReceiver for MockCmdReceiver {
    fn get_command_from_player(&mut self, _: Player) -> Result<Command, Error> {
        let str_cmd = self.commands[self.cursor];
        self.cursor += 1;
        Ok(Command::try_from(str_cmd)?)
    }
}

impl MockCmdReceiver {
    fn x_win() -> Self {
        Self {
            commands: vec!["turn 0", "turn 3", "turn 1", "turn 4", "turn 2"],
            cursor: 0,
        }
    }
    fn o_win() -> Self {
        Self {
            commands: vec!["turn 0", "turn 2", "turn 1", "turn 4", "turn 3", "turn 6"],
            cursor: 0,
        }
    }

    fn x_surrender() -> Self {
        Self {
            commands: vec!["surrender"],
            cursor: 0,
        }
    }

    fn o_surrender() -> Self {
        Self {
            commands: vec!["turn 0", "surrender"],
            cursor: 0,
        }
    }

    fn draw() -> Self {
        Self {
            commands: vec![
                "turn 0", "turn 1", "turn 2", "turn 4", "turn 3", "turn 6", "turn 5", "turn 8",
                "turn 7",
            ],
            cursor: 0,
        }
    }
}
