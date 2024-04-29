mod inline;
mod types;
mod utils;

use crate::parcom::inline::handle_inline;
pub use crate::parcom::types::*;
use crate::Error;
use crate::Error::ParComError;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::char;
use nom::combinator::fail;
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

    let mut el = Element::new(matched_tag);
    el.nodes = nodes;

    Ok((i, Node::Element(el)))
}

fn blockquote(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("bq")(i)?;
    let (i, _) = tag(". ")(i)?;
    let (i, matched_content) = take_until_block_ending(i)?;
    let (_, nodes) = handle_inline(matched_content)?;
    let mut bq = Element::new(Tag::Blockquote);
    let mut p = Element::new(Tag::Paragraph);
    p.nodes = nodes;
    bq.nodes = vec![Node::Element(p)];
    Ok((i, Node::Element(bq)))
}

fn implicit_block(i: &str) -> IResult<&str, Node> {
    let (i, matched_content) = take_until_block_ending(i)?;
    let (_, nodes) = handle_inline(matched_content)?;

    let mut el = Element::new(Tag::Paragraph);
    el.nodes = nodes;
    Ok((i, Node::Element(el)))
}

fn footnote_plain(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("fn")(i)?;
    let (i, matched) = take_while1(|chr: char| chr.is_ascii_digit())(i)?;
    let (i, _) = tag(". ")(i)?;
    let (i, matched_content) = take_until_block_ending(i)?;
    let (_, mut nodes) = handle_inline(matched_content)?;
    let mut el = Element::new(Tag::Footnote);
    el.attrs.classes.push("footnote".to_string());
    el.attrs.id = Some("fn".to_string());
    el.nodes.append(&mut nodes);
    Ok((i, Node::Element(el)))
}

fn footnote_link(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("fn")(i)?;
    let (i, matched) = take_while1(|chr: char| chr.is_ascii_digit())(i)?;
    let (i, _) = tag("^. ")(i)?;
    let (i, matched_content) = take_until_block_ending(i)?;
    let (_, mut nodes) = handle_inline(matched_content)?;
    let mut link_up_attrs = Attributes::new();
    link_up_attrs.href = Some("#fnrev".to_owned());
    let link_up = Element::init(
        Tag::Anchor,
        link_up_attrs,
        vec![Node::Plain(Plain::new(matched))],
    );
    let superscript = Element::init(
        Tag::FootnoteId,
        Attributes::new(),
        vec![Node::Element(link_up)],
    );
    let mut attrs = Attributes::new();
    attrs.classes.push("footnote".to_string());
    attrs.id = Some("fn".to_string());
    let mut el = Element::init(
        Tag::Footnote,
        attrs,
        vec![Node::Element(superscript), Node::Plain(Plain::new(" "))],
    );
    el.nodes.append(&mut nodes);
    Ok((i, Node::Element(el)))
}

fn footnote(i: &str) -> IResult<&str, Node> {
    alt((footnote_plain, footnote_link))(i)
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

            let mut doc = Element::empty(Tag::Doc);
            doc.nodes = nodes;
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
