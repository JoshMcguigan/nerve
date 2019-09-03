use crate::OptimizedCommand;

pub fn emit_asm(commands: &[OptimizedCommand]) -> String {
    let mut s = String::new();

    s += "section .bss\n";
    // define buffer of 30_000 bytes, initialized to zero
    s += "\tbuffer: resb 30000\n\n";

    s += "section .text\n";
    s += "\tglobal _start\n\n";

    s += "_start:\n";

    // store pointer to buffer in register R8
    s += "\tmov R8, buffer\n";

    s = emit_asm_for_commands(s, &commands, 0).0;

    // exit with status code 0
    s += "\tmov rdi, 0\n";
    s += "\tmov rax, 60\n";
    s += "\tsyscall";

    s
}

fn emit_asm_for_commands(
    mut s: String,
    commands: &[OptimizedCommand],
    mut next_loop_number: u16,
) -> (String, u16) {
    for command in commands {
        match command {
            OptimizedCommand::IncrementCell(value) => s += &format!("\tadd byte[R8], {}\n", value),
            OptimizedCommand::IncrementCellAtOffset(offset, value) => {
                s += &format!("\tadd byte[R8 + {}], {}\n", offset, value)
            }
            OptimizedCommand::DecrementCell(value) => s += &format!("\tsub byte[R8], {}\n", value),
            OptimizedCommand::IncrementPointer(num) => s += &format!("\tadd R8, {}\n", num),
            OptimizedCommand::DecrementPointer(num) => s += &format!("\tsub R8, {}\n", num),
            OptimizedCommand::Input => {
                // use syscall to read a single byte from std in
                s += "\tmov rdi, 0\n"; // 0 = std in
                s += "\tmov rsi, R8\n";
                s += "\tmov rdx, 1\n"; // 1 = read a single byte
                s += "\tmov rax, 0\n"; // 0 = syscall id
                s += "\tsyscall\n";
            }
            OptimizedCommand::Output => {
                // use syscall to write a single byte to std out
                s += "\tmov rdi, 1\n"; // 1 = std out
                s += "\tmov rsi, R8\n";
                s += "\tmov rdx, 1\n"; // 1 = write a single byte
                s += "\tmov rax, 1\n"; // 1 = syscall id
                s += "\tsyscall\n";
            }
            OptimizedCommand::While(commands) => {
                let this_loop_number = next_loop_number;
                next_loop_number += 1;
                s += "\tcmp byte[R8], 0\n";
                s += &format!("\tje loop_{}_end\n", this_loop_number);
                s += &format!("\tloop_{}_start:\n", this_loop_number);
                let res = emit_asm_for_commands(s, commands, next_loop_number);
                s = res.0;
                next_loop_number = res.1;
                s += "\tcmp byte[R8], 0\n";
                s += &format!("\tjne loop_{}_start\n", this_loop_number);
                s += &format!("\tloop_{}_end:\n", this_loop_number);
            }
        };
    }

    (s, next_loop_number)
}
