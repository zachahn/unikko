use thiserror::Error;

mod convenience;
mod error;
mod lexer;
mod parser;
mod renderer;

pub use error::Error;
pub use lexer::{tokenize, Token};
pub use parser::{parse, Node};

pub fn textile_to_html(textile: String) -> Result<String, Error> {
    convenience::textile_to_html(textile)
}
