use crate::{CompilerError, Result};
use std::{
    fs::{write, File},
    io::Read,
    path::Path,
};

pub fn read_source(source_file_path: &Path) -> Result<Vec<u8>> {
    let mut f = File::open(source_file_path).map_err(|_| CompilerError::SourceFileRead)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)
        .map_err(|_| CompilerError::SourceFileRead)?;

    Ok(buffer)
}

pub fn write_to_file(asm: String, file_path: &Path) -> Result<()> {
    write(file_path, asm).map_err(|_| CompilerError::OutputFileWrite)
}
