use thiserror::Error;

mod lexer;
mod parser;

pub use lexer::tokenize;
pub use parser::{parse, Node};

#[derive(Error, Debug)]
pub enum UnikkoError {
    #[error("the lexer broke")]
    LexerError,
    #[error("the parser broke")]
    ParserError,
}
