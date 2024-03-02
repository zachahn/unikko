use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("the lexer broke")]
    LexerError,
    #[error("the parser broke")]
    ParserError,
}
