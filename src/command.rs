use crate::{CompilerError, Result};

#[derive(PartialEq)]
pub enum Command {
    IncrementCell,
    DecrementCell,
    IncrementPointer,
    DecrementPointer,
    While(Vec<Command>),
    Output,
    Input,
}

pub fn parse(source_code: Vec<u8>) -> Result<Vec<Command>> {
    let (commands, remaining_source) = parse_block(&source_code);
    if remaining_source.is_empty() {
        Ok(commands)
    } else {
        Err(CompilerError::UnmatchedBracket)
    }
}

fn parse_block(mut remaining_source: &[u8]) -> (Vec<Command>, &[u8]) {
    let mut commands = vec![];
    while let Some((byte, rem)) = remaining_source.split_first() {
        remaining_source = rem;
        let command = match byte {
            b'+' => Command::IncrementCell,
            b'-' => Command::DecrementCell,
            b'>' => Command::IncrementPointer,
            b'<' => Command::DecrementPointer,
            b'[' => {
                let (commands_in_the_loop, rem) = parse_block(remaining_source);
                remaining_source = rem;
                Command::While(commands_in_the_loop)
            }
            b']' => break,
            b'.' => Command::Output,
            b',' => Command::Input,
            _ => continue, // all other byte patterns ignored per language spec
        };

        commands.push(command);
    }

    (commands, remaining_source)
}
