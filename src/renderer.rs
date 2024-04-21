use crate::parser::Node;
use crate::Error;

fn recursively_render(buffer: &mut String, node: Node) -> Result<(), crate::Error> {
    match node {
        Node::Document(doc) => {
            for child in doc.nodes {
                recursively_render(buffer, child)?
            }
        }
        Node::Element(element) => {
            buffer.push_str(format!("<{}>", element.identifier).as_str());
            for child in element.nodes {
                recursively_render(buffer, child)?
            }
            buffer.push_str(format!("</{}>", element.identifier).as_str());
        }
        Node::Plain(plain) => buffer.push_str(plain.content.as_str()),
        Node::NewLine => {
            buffer.push_str("\n");
        }
    }
    Ok(())
}

pub fn render(node: Node) -> Result<String, Error> {
    let mut buffer = String::new();
    recursively_render(&mut buffer, node)?;
    Ok(buffer)
}
