use crate::parser::{Node, Tag};
use crate::Error;
use crate::Options;

fn recursively_render(
    buffer: &mut String,
    options: &Options,
    node: Node,
) -> Result<(), crate::Error> {
    match node {
        Node::Element(element) => {
            if element.identifier == Tag::Doc {
                for child in element.nodes {
                    recursively_render(buffer, options, child)?
                }
                return Ok(());
            }
            let tag = match element.identifier {
                Tag::Bq => "blockquote".to_string(),
                other => other.to_string(),
            };
            buffer.push_str(format!("<{}>", tag).as_str());
            for child in element.nodes {
                recursively_render(buffer, options, child)?
            }
            buffer.push_str(format!("</{}>", tag).as_str());
        }
        Node::Plain(plain) => buffer.push_str(plain.content.as_str()),
        Node::NewLine => {
            buffer.push_str("\n");
        }
        Node::Symbol(symbol) => {
            if let Some(replacement) = options.symbols.get(&symbol) {
                buffer.push_str(replacement);
            }
        }
    }
    Ok(())
}

pub fn render(node: Node, options: &Options) -> Result<String, Error> {
    let mut buffer = String::new();
    recursively_render(&mut buffer, options, node)?;
    Ok(buffer)
}
