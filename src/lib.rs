use thiserror::Error;

mod convenience;
mod error;
mod options;
mod pulp {
    mod pass_1;
}

pub use error::Error;
pub use options::Options;

// pub fn textile_to_html_with_options(textile: String, options: Options) -> Result<String, Error> {
//     convenience::textile_to_html_with_options(textile, options)
// }

// pub fn textile_to_html(textile: String) -> Result<String, Error> {
//     convenience::textile_to_html(textile)
// }

// pub fn textile_to_tree_with_options(
//     textile: String,
//     options: Options,
// ) -> Result<parcom::Node, Error> {
//     convenience::textile_to_tree_with_options(textile, options)
// }

// pub fn textile_to_tree(textile: String) -> Result<parcom::Node, Error> {
//     convenience::textile_to_tree(textile)
// }
