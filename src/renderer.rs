use crate::parcom::{Element, Node, Tag};
use crate::Error;
use crate::Options;

fn tag_name(tag: &Tag) -> &str {
    match tag {
        Tag::Bq => "blockquote",
        Tag::P => "p",
        Tag::H1 => "h1",
        Tag::H2 => "h2",
        Tag::H3 => "h3",
        Tag::H4 => "h4",
        Tag::H5 => "h5",
        Tag::H6 => "h6",
        Tag::A => "a",
        Tag::B => "b",
        Tag::Strong => "strong",
        Tag::I => "i",
        Tag::Em => "em",
        Tag::Span => "span",
        Tag::Other(_) => unimplemented!(),
        Tag::Doc => unimplemented!(),
    }
}

fn opening_tag(element: &Element, options: &Options) -> String {
    let mut buffer = format!("<{}", tag_name(&element.tag));
    if let Some(ref href) = element.attrs.href {
        buffer.push_str(format!(" href=\"{}\"", href).as_str());
    }
    if element.attrs.classes.len() > 0 {
        buffer.push_str(format!(" class=\"{}\"", element.attrs.classes.join(" ")).as_str());
    }
    buffer.push_str(">");
    return buffer;
}

fn closing_tag(element: &Element, _options: &Options) -> String {
    let buffer = format!("</{}>", tag_name(&element.tag));
    return buffer;
}

fn recursively_render(
    buffer: &mut String,
    options: &Options,
    node: &Node,
) -> Result<(), crate::Error> {
    match node {
        Node::Element(element) => {
            if element.tag == Tag::Doc {
                for child in &element.nodes {
                    recursively_render(buffer, options, child)?
                }
                return Ok(());
            }
            buffer.push_str(opening_tag(&element, options).as_str());
            for child in &element.nodes {
                recursively_render(buffer, options, child)?
            }
            buffer.push_str(closing_tag(&element, options).as_str());
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

pub fn render(node: &Node, options: &Options) -> Result<String, Error> {
    let mut buffer = String::new();
    recursively_render(&mut buffer, options, node)?;
    Ok(buffer)
}
