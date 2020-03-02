use crate::OptimizedCommand;

pub fn apply(commands: Vec<OptimizedCommand>) -> Vec<OptimizedCommand> {
    let mut optimized_commands = vec![];
    let mut i = 0;

    while let Some(command1) = commands.get(i) {
        let mut output = match (command1, commands.get(i + 1), commands.get(i + 2)) {
            (
                OptimizedCommand::IncrementPointer(pointer_increment),
                Some(OptimizedCommand::IncrementCell(cell_increment)),
                Some(OptimizedCommand::DecrementPointer(pointer_decrement)),
            ) => {
                let mut out = vec![OptimizedCommand::IncrementCellAtOffset(
                    *pointer_increment,
                    *cell_increment,
                )];
                match pointer_increment.cmp(pointer_decrement) {
                    std::cmp::Ordering::Less => {
                        out.push(OptimizedCommand::DecrementPointer(
                            pointer_decrement - pointer_increment,
                        ));
                    }
                    // If the increment and decrement were equal, we
                    // don't need to move the pointer at all
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => {
                        out.push(OptimizedCommand::IncrementPointer(
                            pointer_increment - pointer_decrement,
                        ));
                    }
                }

                // in this case we are going to handle three instructions
                i += 3;

                out
            }
            (command, _, _) => {
                // in this branch we are handling a single instruction
                i += 1;
                vec![command.clone()]
            }
        };

        optimized_commands.append(&mut output);
    }

    optimized_commands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_through_short_instruction_set() {
        let input = vec![OptimizedCommand::IncrementCell(1)];
        let expected_output = input.clone();

        assert_eq!(expected_output, apply(input));
    }

    #[test]
    fn pass_through_non_relevant_instruction_set() {
        let input = vec![
            OptimizedCommand::Output,
            OptimizedCommand::Output,
            OptimizedCommand::Output,
        ];
        let expected_output = input.clone();

        assert_eq!(expected_output, apply(input));
    }

    #[test]
    fn equal_shift_of_one_increment_cell_by_one() {
        let input = vec![
            OptimizedCommand::IncrementPointer(1),
            OptimizedCommand::IncrementCell(1),
            OptimizedCommand::DecrementPointer(1),
        ];
        let expected_output = vec![OptimizedCommand::IncrementCellAtOffset(1, 1)];

        assert_eq!(expected_output, apply(input));
    }

    #[test]
    fn in_middle_of_program() {
        let input = vec![
            OptimizedCommand::Output,
            OptimizedCommand::IncrementPointer(1),
            OptimizedCommand::IncrementCell(1),
            OptimizedCommand::DecrementPointer(1),
            OptimizedCommand::Output,
            OptimizedCommand::Output,
            OptimizedCommand::Output,
        ];
        let expected_output = vec![
            OptimizedCommand::Output,
            OptimizedCommand::IncrementCellAtOffset(1, 1),
            OptimizedCommand::Output,
            OptimizedCommand::Output,
            OptimizedCommand::Output,
        ];

        assert_eq!(expected_output, apply(input));
    }

    #[test]
    fn near_end_of_program() {
        let input = vec![
            OptimizedCommand::Output,
            OptimizedCommand::IncrementPointer(1),
            OptimizedCommand::IncrementCell(1),
            OptimizedCommand::DecrementPointer(1),
            OptimizedCommand::Output,
        ];
        let expected_output = vec![
            OptimizedCommand::Output,
            OptimizedCommand::IncrementCellAtOffset(1, 1),
            OptimizedCommand::Output,
        ];

        assert_eq!(expected_output, apply(input));
    }

    #[test]
    fn equal_shift_of_one_increment_cell_by_two() {
        let input = vec![
            OptimizedCommand::IncrementPointer(1),
            OptimizedCommand::IncrementCell(2),
            OptimizedCommand::DecrementPointer(1),
        ];
        let expected_output = vec![OptimizedCommand::IncrementCellAtOffset(1, 2)];

        assert_eq!(expected_output, apply(input));
    }

    #[test]
    fn equal_shift_of_two_increment_cell_by_one() {
        let input = vec![
            OptimizedCommand::IncrementPointer(2),
            OptimizedCommand::IncrementCell(1),
            OptimizedCommand::DecrementPointer(2),
        ];
        let expected_output = vec![OptimizedCommand::IncrementCellAtOffset(2, 1)];

        assert_eq!(expected_output, apply(input));
    }

    #[test]
    fn net_positive_shift() {
        let input = vec![
            OptimizedCommand::IncrementPointer(3),
            OptimizedCommand::IncrementCell(1),
            OptimizedCommand::DecrementPointer(2),
        ];
        let expected_output = vec![
            OptimizedCommand::IncrementCellAtOffset(3, 1),
            OptimizedCommand::IncrementPointer(1),
        ];

        assert_eq!(expected_output, apply(input));
    }

    #[test]
    fn net_negative_shift() {
        let input = vec![
            OptimizedCommand::IncrementPointer(2),
            OptimizedCommand::IncrementCell(1),
            OptimizedCommand::DecrementPointer(3),
        ];
        let expected_output = vec![
            OptimizedCommand::IncrementCellAtOffset(2, 1),
            OptimizedCommand::DecrementPointer(1),
        ];

        assert_eq!(expected_output, apply(input));
    }

    // TODO handle decrement pointer followed by increment pointer
    // TODO handle decrement cell
}
