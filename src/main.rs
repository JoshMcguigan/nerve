use std::{
    fs,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, CompilerError>;

fn main() -> Result<()> {
    let config: CompilerConfig = read_config()?;
    let source_code: Vec<u8> = read_source(&config.source_file_path)?;
    let commands: Vec<Command> = parse(source_code)?;
    let optimized_commands : Vec<OptimizedCommand> = optimize(&commands);
    let asm: String = emit_asm(&optimized_commands);
    write_to_file(asm, &config.output_file_path())?;

    Ok(())
}

#[derive(Debug)]
enum CompilerError {
    MissingArg,
    SourceFileRead,
    UnmatchedBracket,
    OutputFileWrite,
}

struct CompilerConfig {
    source_file_path: PathBuf,
}

enum Command {
    IncrementCell,
    DecrementCell,
    IncrementPointer,
    DecrementPointer,
    While(Vec<Command>),
    Output,
    Input,
}

enum OptimizedCommand {
    IncrementCell(u16),
    DecrementCell(u16),
    IncrementPointer(u16),
    DecrementPointer(u16),
    While(Vec<OptimizedCommand>),
    Output,
    Input,
}

fn read_config() -> Result<CompilerConfig> {
    let file_path = std::env::args().nth(1).ok_or(CompilerError::MissingArg)?;
    Ok(CompilerConfig {
        source_file_path: PathBuf::from(file_path),
    })
}

fn read_source(source_file_path: &Path) -> Result<Vec<u8>> {
    let mut f = File::open(source_file_path).map_err(|_| CompilerError::SourceFileRead)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)
        .map_err(|_| CompilerError::SourceFileRead)?;

    Ok(buffer)
}

fn parse(source_code: Vec<u8>) -> Result<Vec<Command>> {
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

fn optimize(commands: &[Command]) -> Vec<OptimizedCommand> {
    let mut optimized_commands = vec![];

    for command in commands {
        let optimized_command = match command {
            Command::IncrementCell => OptimizedCommand::IncrementCell(1),
            Command::DecrementCell => OptimizedCommand::DecrementCell(1),
            Command::IncrementPointer => OptimizedCommand::IncrementPointer(1),
            Command::DecrementPointer => OptimizedCommand::DecrementPointer(1),
            Command::While(commands) => OptimizedCommand::While(optimize(&commands)),
            Command::Output => OptimizedCommand::Output,
            Command::Input => OptimizedCommand::Input,
        };

        optimized_commands.push(optimized_command);
    }

    optimized_commands
}

fn emit_asm(commands: &[OptimizedCommand]) -> String {
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
            OptimizedCommand::IncrementPointer(num) => s += &format!("\tadd R8, {}\n", num),
            OptimizedCommand::DecrementPointer(num) => s += "\tdec R8\n",
            OptimizedCommand::IncrementCell(num) => s += "\tinc byte[R8]\n",
            OptimizedCommand::DecrementCell(num) => s += "\tdec byte[R8]\n",
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

fn write_to_file(asm: String, file_path: &Path) -> Result<()> {
    fs::write(file_path, asm).map_err(|_| CompilerError::OutputFileWrite)
}

impl CompilerConfig {
    fn output_file_path(&self) -> PathBuf {
        let mut output_file_path = self.source_file_path.clone();
        output_file_path.set_extension("s");
        output_file_path
    }
}
