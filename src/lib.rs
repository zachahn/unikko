use thiserror::Error;

mod convenience;
mod error;
mod options;
mod pulp;
mod renderer;

pub use error::Error;
pub use options::Options;

pub fn textile_to_html_with_options<'a>(
    textile: &'a str,
    options: Options,
) -> Result<String, Error> {
    convenience::textile_to_html_with_options(textile, options)
}

pub fn textile_to_html<'a>(textile: &'a str) -> Result<String, Error> {
    convenience::textile_to_html_with_options(textile, Options::default())
}
