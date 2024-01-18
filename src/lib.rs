use thiserror::Error;

mod lexer;
mod parser;

pub use lexer::tokenize;
pub use parser::{parse, Node, Token};

#[derive(Error, Debug)]
pub enum Error {
    #[error("the parser broke")]
    ParserError,
}
