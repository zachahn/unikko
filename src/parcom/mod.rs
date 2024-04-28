mod types;

use crate::options::Symbol;
pub use crate::parcom::types::*;
use crate::Error;
use crate::Error::ParComError;

use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_until, take_while1};
use nom::character::complete::char;
use nom::combinator::all_consuming;
use nom::combinator::fail;
use nom::multi::{many0, many1};
use nom::IResult;

#[allow(dead_code)]
fn dbg_dmp_s<'a, F, O, E: std::fmt::Debug>(
    mut f: F,
    context: &'static str,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    use nom::HexDisplay;
    move |i: &'a str| match f(i) {
        Err(e) => {
            println!("{}: Error({:?}) at:\n{}", context, e, i.to_hex(8));
            Err(e)
        }
        a => a,
    }
}

fn is_plain(chr: char) -> bool {
    match chr {
        'a'..='z' => true,
        'A'..='Z' => true,
        '0'..='9' => true,
        ' ' => true,
        '.' | '?' => true,
        _ => false,
    }
}

fn plain(i: &str) -> IResult<&str, Node> {
    let (i, matched) = take_while1(is_plain)(i)?;
    Ok((i, Node::Plain(Plain::new(matched))))
}

fn catchall1(i: &str) -> IResult<&str, Node> {
    let (i, matched) = take(1usize)(i)?;
    Ok((i, Node::Plain(Plain::new(matched))))
}

fn newline(i: &str) -> IResult<&str, Node> {
    let (i, _) = char('\n')(i)?;
    Ok((i, Node::NewLine))
}

fn bold(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("**")(i)?;
    let (i, inside) = take_until("*")(i)?;
    let (i, _) = tag("**")(i)?;
    let (_, nodes) = inline(inside)?;
    let mut element = Element::empty(Tag::B);
    element.nodes = nodes;
    Ok((i, Node::Element(element)))
}

fn strong(i: &str) -> IResult<&str, Node> {
    let (i, _) = char('*')(i)?;
    let (i, inside) = take_until("*")(i)?;
    let (i, _) = char('*')(i)?;
    let (_, node) = plain(inside)?;
    let mut element = Element::empty(Tag::Strong);
    element.nodes.push(node);
    Ok((i, Node::Element(element)))
}

fn is_url(chr: char) -> bool {
    match chr {
        'a'..='z' => true,
        'A'..='Z' => true,
        '0'..='9' => true,
        ':' => true,
        '/' => true,
        '.' => true,
        '?' | '&' => true,
        '%' => true,
        _ => false,
    }
}

fn link(i: &str) -> IResult<&str, Node> {
    let (i, _) = char('"')(i)?;
    let (i, display) = take_until("\"")(i)?;
    let (i, _) = tag("\":")(i)?;
    let (i, url) = take_while1(is_url)(i)?;
    let mut el = Element::new(Tag::A, false);
    el.nodes = vec![Node::Plain(Plain::new(display))];
    el.attrs.href = Some(url.to_string());

    Ok((i, Node::Element(el)))
}

fn apostrophe(i: &str) -> IResult<&str, Node> {
    let (i, _) = char('\'')(i)?;

    Ok((i, Node::Symbol(Symbol::Apostrophe)))
}

fn inline(i: &str) -> IResult<&str, Vec<Node>> {
    let alts = (bold, strong, plain, apostrophe, link, newline, catchall1);
    many1(alt(alts))(i)
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
    let (_, nodes) = all_consuming(inline)(matched_content)?;

    let mut el = Element::new(matched_tag, false);
    el.nodes = nodes;

    Ok((i, Node::Element(el)))
}

fn blockquote(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("bq")(i)?;
    let (i, _) = tag(". ")(i)?;
    let (i, matched_content) = take_until_block_ending(i)?;
    let (_, nodes) = inline(matched_content)?;
    let mut bq = Element::new(Tag::Bq, false);
    let mut p = Element::new(Tag::P, false);
    p.nodes = nodes;
    bq.nodes = vec![Node::Element(p)];
    Ok((i, Node::Element(bq)))
}

fn implicit_block(i: &str) -> IResult<&str, Node> {
    let (i, matched_content) = take_until_block_ending(i)?;
    let (_, nodes) = all_consuming(inline)(matched_content)?;

    let mut el = Element::new(Tag::P, false);
    el.nodes = nodes;
    Ok((i, Node::Element(el)))
}

fn doc_fragment(i: &str) -> IResult<&str, Node> {
    let alts = (newline, blockquote, explicit_block, implicit_block);
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
    fn plain1() -> Result<()> {
        let input = "but why male models?";
        let (remaining, node) = plain(input)?;
        println!("🔎 {:?}", node);
        assert_eq!(remaining, "");
        Ok(())
    }

    #[test]
    fn inline1() -> Result<()> {
        let input = "they're in the computer";
        let (remaining, node) = inline(input)?;
        println!("🔎 {:?}", node);
        assert_eq!(remaining, "");
        Ok(())
    }

    #[test]
    fn inline2() -> Result<()> {
        let input = "*hi* **hello** *hi*";
        let (remaining, node) = inline(input)?;
        println!("🔎 {:?}", node);
        assert_eq!(remaining, "");
        Ok(())
    }

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
    fn strong1() -> Result<()> {
        let input = "*testing testing*";
        let (remaining_input, element) = strong(input)?;
        println!("🔎 {:?}", element);
        assert_eq!("", remaining_input);
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
