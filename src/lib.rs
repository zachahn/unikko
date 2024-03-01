use thiserror::Error;

mod lexer;
mod modifiers;
mod parser;

pub use lexer::{tokenize, Token};
pub use parser::{parse, Node};

#[derive(Error, Debug)]
pub enum UnikkoError {
    #[error("the lexer broke")]
    LexerError,
    #[error("the parser broke")]
    ParserError,
}
