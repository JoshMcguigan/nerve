use crate::{CompilerError, Result};
use std::path::PathBuf;

pub struct CompilerConfig {
    pub source_file_path: PathBuf,
}

impl CompilerConfig {
    pub fn from_args() -> Result<CompilerConfig> {
        let file_path = std::env::args().nth(1).ok_or(CompilerError::MissingArg)?;
        Ok(CompilerConfig {
            source_file_path: PathBuf::from(file_path),
        })
    }

    pub fn output_file_path(&self) -> PathBuf {
        let mut output_file_path = self.source_file_path.clone();
        output_file_path.set_extension("s");
        output_file_path
    }
}
