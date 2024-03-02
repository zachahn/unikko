use thiserror::Error;

mod convenience;
mod lexer;
mod modifiers;
mod parser;
mod renderer;

pub use lexer::{tokenize, Token};
pub use parser::{parse, Node};

pub fn textile_to_html(textile: String) -> Result<String, Error> {
    convenience::textile_to_html(textile)
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("the lexer broke")]
    LexerError,
    #[error("the parser broke")]
    ParserError,
}
