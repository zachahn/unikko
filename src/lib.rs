use thiserror::Error;

mod convenience;
mod error;
mod lexer;
mod options;
mod parcom;
mod parser;
mod renderer;

pub use error::Error;
pub use lexer::{tokenize, Token};
pub use options::Options;
pub use parser::{parse, Node};

pub fn textile_to_html_with_options(textile: String, options: Options) -> Result<String, Error> {
    convenience::textile_to_html_with_options(textile, options)
}

pub fn textile_to_html(textile: String) -> Result<String, Error> {
    convenience::textile_to_html(textile)
}
