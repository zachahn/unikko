use crate::pulp::Event;
use crate::Error;
use crate::Options;

pub fn render<'a, I>(iterator: &mut I, _options: &Options) -> Result<String, Error>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut buffer = String::new();
    for event in iterator {
        match event {
            Event::Paragraph => buffer.push_str("<p>"),
            Event::ParagraphEnd => buffer.push_str("</p>"),
            Event::Text(_, _, text) => buffer.push_str(text),
            Event::LineBreak(_, _) => buffer.push_str("<br>\n"),
        }
    }
    Ok(buffer)
}
