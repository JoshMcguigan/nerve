pub type Result<T> = std::result::Result<T, CompilerError>;

#[derive(Debug)]
pub enum CompilerError {
    MissingArg,
    SourceFileRead,
    UnmatchedBracket,
    OutputFileWrite,
}
