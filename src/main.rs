mod asm;
use asm::emit_asm;

mod command;
use command::{parse, Command};

mod config;
use config::CompilerConfig;

mod error;
use error::{CompilerError, Result};

mod fs;
use fs::{read_source, write_to_file};

mod optimized_command;
use optimized_command::{optimize, OptimizedCommand};

fn main() -> Result<()> {
    let config = CompilerConfig::from_args()?;
    let source_code: Vec<u8> = read_source(&config.source_file_path)?;
    let commands: Vec<Command> = parse(source_code)?;
    let optimized_commands: Vec<OptimizedCommand> = optimize(&commands);
    let asm: String = emit_asm(&optimized_commands);
    write_to_file(asm, &config.output_file_path())?;

    Ok(())
}
