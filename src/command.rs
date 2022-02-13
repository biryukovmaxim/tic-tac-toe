use crate::Error;

pub enum Command {
    Turn(usize),
    Surrender,
}

impl TryFrom<&str> for Command {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chunks_iter = value.split_whitespace();
        if let Some(mb_command) = chunks_iter.next() {
            match mb_command {
                "surrender" => Ok(Command::Surrender),
                "turn" => chunks_iter
                    .next()
                    .ok_or_else(|| Error::ParseCommandError(value.to_owned()))?
                    .parse::<usize>()
                    .map_or_else(
                        |_| Err(Error::ParseCommandError(value.to_owned())),
                        |v| Ok(Command::Turn(v)),
                    ),

                _ => Err(Error::ParseCommandError(value.to_owned())),
            }
        } else {
            Err(Error::ParseCommandError(value.to_owned()))
        }
    }
}
