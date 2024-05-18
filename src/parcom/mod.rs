mod inline;
mod types;
mod utils;

use crate::parcom::inline::handle_inline;
pub use crate::parcom::types::*;
use crate::Error;
use crate::Error::ParComError;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::char;
use nom::combinator::fail;
use nom::combinator::opt;
use nom::multi::many0;
use nom::IResult;

fn newline(i: &str) -> IResult<&str, Node> {
    let (i, _) = char('\n')(i)?;
    Ok((i, Node::NewLine))
}

fn take_until_block_ending(i: &str) -> IResult<&str, &str> {
    if i.is_empty() {
        return fail(i);
    }

    if let Some(location) = i.find("\n\n") {
        let remaining = i.get(location..).unwrap();
        let matched = i.get(0..location).unwrap();
        return Ok((remaining, matched));
    }

    if let Some(location) = i.rfind("\n") {
        let remaining = i.get(location..).unwrap();
        if remaining == "\n" {
            let matched = i.get(0..location).unwrap();
            return Ok((remaining, matched));
        }
    }

    Ok(("", i))
}

fn explicit_block(i: &str) -> IResult<&str, Node> {
    let acceptable_tags = (
        tag("p"),
        tag("h1"),
        tag("h2"),
        tag("h3"),
        tag("h4"),
        tag("h5"),
        tag("h6"),
    );
    let (i, matched_tag) = alt(acceptable_tags)(i)?;
    let (i, _) = tag(". ")(i)?;
    let (i, matched_content) = take_until_block_ending(i)?;
    let (_, nodes) = handle_inline(matched_content)?;
    let el = Element::nodes(Tag::try_from(matched_tag).unwrap(), nodes);
    Ok((i, Node::Element(el)))
}

fn blockquote(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("bq")(i)?;
    let (i, _) = tag(". ")(i)?;
    let (i, matched_content) = take_until_block_ending(i)?;
    let (_, nodes) = handle_inline(matched_content)?;
    let p = Element::nodes(Tag::Paragraph, nodes);
    let bq = Element::nodes(Tag::Blockquote, vec![Node::Element(p)]);
    Ok((i, Node::Element(bq)))
}

fn implicit_block(i: &str) -> IResult<&str, Node> {
    let (i, matched_content) = take_until_block_ending(i)?;
    let (_, nodes) = handle_inline(matched_content)?;

    let el = Element::nodes(Tag::Paragraph, nodes);
    Ok((i, Node::Element(el)))
}

fn classes_ids<'a, 'b>(i: &'a str, attrs: &'b mut Attributes) -> IResult<&'a str, ()> {
    let (i, _) = tag("(")(i)?;
    let (i, caught) = take_while1(|chr: char| !chr.is_control() && chr != ')')(i)?;
    let (i, _) = tag(")")(i)?;
    let (ids, classes) = opt(take_while(|chr: char| chr != '#'))(caught)?;
    for class in classes.unwrap_or("").split(" ") {
        attrs.classes.push(class.into());
    }
    for id in ids.split("#") {
        if id == "" {
            continue;
        }
        attrs.id = Some(id.into())
    }
    Ok((i, ()))
}

fn attributes(i: &str) -> IResult<&str, Attributes> {
    let mut attrs = Attributes::new();
    let (i, _) = classes_ids(i, &mut attrs).unwrap_or_else(|_| (i, ()));
    Ok((i, attrs))
}

fn footnote(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("fn")(i)?;
    let (i, matched) = take_while1(|chr: char| chr.is_ascii_digit())(i)?;
    let (i, superscript_content) = if i.starts_with("^") {
        let link_up = Element::attrs_nodes(
            Tag::Anchor,
            Attributes::href("#fnrev".into()),
            vec![Node::Plain(Plain::new(matched))],
        );
        (&i[1..], Node::Element(link_up))
    } else {
        (i, Node::Plain(Plain::new(matched)))
    };
    let (i, mut el_attrs) = attributes(i)?;
    let (i, _) = tag(". ")(i)?;
    let (i, matched_content) = take_until_block_ending(i)?;
    let (_, mut nodes) = handle_inline(matched_content)?;
    let mut superscript_attrs = Attributes::new();
    if el_attrs.classes.is_empty() {
        el_attrs.classes.push("footnote".into())
    }
    if matches!(el_attrs.id, None) {
        el_attrs.id = Some("fn".into())
    } else {
        superscript_attrs.id = Some("fn".into())
    }
    let superscript = Element::attrs_nodes(
        Tag::FootnoteId,
        superscript_attrs,
        vec![superscript_content],
    );
    let mut el = Element::attrs_nodes(
        Tag::Footnote,
        el_attrs,
        vec![Node::Element(superscript), Node::Plain(Plain::new(" "))],
    );
    el.nodes.append(&mut nodes);
    Ok((i, Node::Element(el)))
}

fn doc_fragment(i: &str) -> IResult<&str, Node> {
    let alts = (
        newline,
        blockquote,
        footnote,
        explicit_block,
        implicit_block,
    );
    alt(alts)(i)
}

pub fn parcom(i: &str) -> Result<Node, Error> {
    match many0(doc_fragment)(i) {
        Err(x) => {
            println!("🔎 {:?}", x);
            return Err(ParComError {
                msg: "parcom error",
            });
        }
        Ok((i, nodes)) => {
            if i != "" {
                println!("🎁 {:?}", i);
                return Err(ParComError {
                    msg: "Nonparsed fragment",
                });
            }

            let doc = Element::nodes(Tag::Doc, nodes);
            return Ok(Node::Element(doc));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn take_until_block_ending1() -> Result<()> {
        assert!(matches!(take_until_block_ending(""), Err(_)));
        Ok(())
    }

    #[test]
    fn integration1() -> Result<()> {
        let input = "bq. Don't suck the **brown stuff(tm)** off of \"2 pence\":http://royalmint.gov.uk coins; it ain't chocolate.";
        let doc = parcom(input)?;
        println!("🔎 {:?}", doc);
        Ok(())
    }

    #[test]
    fn integration2() -> Result<()> {
        let input = "\n\n";
        let doc = parcom(input)?;
        println!("🔎 {:?}", doc);
        Ok(())
    }

    #[test]
    fn ensure_no_match_is_error() -> Result<()> {
        assert!(matches!(blockquote(""), Err(_)));
        assert!(matches!(explicit_block(""), Err(_)));
        assert!(matches!(implicit_block(""), Err(_)));
        assert!(matches!(newline(""), Err(_)));
        assert!(matches!(doc_fragment(""), Err(_)));
        Ok(())
    }

    #[test]
    fn explicit_block1() -> Result<()> {
        let input = "p. hello\n";
        let (remaining_input, element) = explicit_block(input)?;
        println!("🔎 {:?}", element);
        assert_eq!("\n", remaining_input);
        Ok(())
    }
}
