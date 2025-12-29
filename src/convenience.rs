use crate::pulp::Parser;
use crate::renderer::render;
use crate::Error;
use crate::Options;

pub fn textile_to_html_with_options<'a>(
    textile: &'a str,
    options: Options,
) -> Result<String, Error> {
    let mut iterator = Parser::new(textile);
    render(&mut iterator, &options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn simple() -> Result<()> {
        assert_eq!(
            "<p>orange</p><p>mocha</p>",
            textile_to_html_with_options("orange\n\nmocha", Options::default())?
        );
        Ok(())
    }
}
