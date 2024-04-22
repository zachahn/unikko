use crate::lexer::tokenize;
use crate::parser::parse;
use crate::renderer::render;
use crate::Error;
use crate::Options;
use std::io::Cursor;

pub fn textile_to_html_with_options(
    textile: impl Into<String>,
    options: Options,
) -> Result<String, Error> {
    let mut input = Cursor::new(textile.into());
    let tokens = tokenize(&mut input, &options)?;
    let tree = parse(tokens, &options)?;
    render(tree, &options)
}

pub fn textile_to_html(textile: impl Into<String>) -> Result<String, Error> {
    textile_to_html_with_options(textile, Options::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn simple() -> Result<()> {
        assert_eq!(
            "<h1>orange</h1>\n\n<p>mocha</p>",
            textile_to_html("h1. orange\n\nmocha")?
        );
        Ok(())
    }
}
