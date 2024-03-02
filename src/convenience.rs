use crate::lexer::tokenize;
use crate::parser::parse;
use crate::renderer::render;
use crate::Error;
use std::io::Cursor;

pub fn textile_to_html<S: Into<String>>(textile: S) -> Result<String, Error> {
    let mut input = Cursor::new(textile.into());
    let tokens = tokenize(&mut input)?;
    let tree = parse(tokens.into())?;
    render(tree)
}
