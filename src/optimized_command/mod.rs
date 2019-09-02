use crate::Command;
use std::{iter::Peekable, slice};

mod shift_elision;

#[derive(Debug, PartialEq, Clone)]
pub enum OptimizedCommand {
    IncrementCell(u16),
    IncrementCellAtOffset(u16, u16),
    DecrementCell(u16),
    IncrementPointer(u16),
    DecrementPointer(u16),
    While(Vec<OptimizedCommand>),
    Output,
    Input,
}

pub fn optimize(commands: &[Command]) -> Vec<OptimizedCommand> {
    let mut optimized_commands = vec![];

    let mut iter = commands.iter().peekable();
    while let Some(command) = iter.next() {
        let optimized_command = match command {
            Command::IncrementCell => {
                let mut num = 1;
                num += consume_commands(Command::IncrementCell, &mut iter);
                OptimizedCommand::IncrementCell(num)
            }
            Command::DecrementCell => {
                let mut num = 1;
                num += consume_commands(Command::DecrementCell, &mut iter);
                OptimizedCommand::DecrementCell(num)
            }
            Command::IncrementPointer => {
                let mut num = 1;
                num += consume_commands(Command::IncrementPointer, &mut iter);
                OptimizedCommand::IncrementPointer(num)
            }
            Command::DecrementPointer => {
                let mut num = 1;
                num += consume_commands(Command::DecrementPointer, &mut iter);
                OptimizedCommand::DecrementPointer(num)
            }
            Command::While(commands) => OptimizedCommand::While(optimize(&commands)),
            Command::Output => OptimizedCommand::Output,
            Command::Input => OptimizedCommand::Input,
        };

        optimized_commands.push(optimized_command);
    }

    optimized_commands = shift_elision::apply(optimized_commands);

    optimized_commands
}

fn consume_commands(command_type: Command, iter: &mut Peekable<slice::Iter<'_, Command>>) -> u16 {
    let mut num = 0;
    while iter.peek() == Some(&&command_type) {
        iter.next();
        num += 1;
    }

    num
}
