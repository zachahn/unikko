use crate::parcom::parcom;
use crate::renderer::render;
use crate::Error;
use crate::Options;

pub fn textile_to_html_with_options<'a>(
    textile: impl AsRef<str>,
    options: Options,
) -> Result<String, Error> {
    let tree = parcom(textile.as_ref())?;
    render(&tree, &options)
}

pub fn textile_to_html<'a>(textile: impl AsRef<str>) -> Result<String, Error> {
    textile_to_html_with_options(textile, Options::default())
}

pub fn textile_to_tree_with_options<'a>(
    textile: impl AsRef<str>,
    options: Options,
) -> Result<crate::parcom::Node, Error> {
    parcom(textile.as_ref())
}

pub fn textile_to_tree<'a>(textile: impl AsRef<str>) -> Result<crate::parcom::Node, Error> {
    textile_to_tree_with_options(textile, Options::default())
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
