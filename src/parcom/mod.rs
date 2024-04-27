mod types;

use crate::options::Symbol;
use crate::parcom::types::*;

use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_till, take_until, take_while1};
use nom::character::complete::char;
use nom::combinator::fail;
use nom::multi::{many0, many1};
use nom::IResult;

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
    char('\n')(i)?;

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
    alt((take_until("\n\n"), take_till(|_| false)))(i)
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
    let (ii, nodes) = inline(matched_content)?;
    if ii != "" {
        fail::<_, &str, _>(ii)?;
    }

    let mut el = Element::new(matched_tag, false);
    el.nodes = nodes;

    Ok((i, Node::Element(el)))
}

fn blockquote(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("bq")(i)?;
    let (i, _) = tag(". ")(i)?;
    let (i, matched_content) = take_until_block_ending(i)?;
    let (ii, nodes) = inline(matched_content)?;
    let mut bq = Element::new(Tag::Bq, false);
    let mut p = Element::new(Tag::P, false);
    p.nodes = nodes;
    bq.nodes = vec![Node::Element(p)];

    Ok((i, Node::Element(bq)))
}

fn parcom(i: &str) -> IResult<&str, Node> {
    let (i, nodes) = many0(alt((blockquote, explicit_block, newline)))(i)?;

    let mut doc = Element::empty(Tag::Doc);
    doc.nodes = nodes;

    Ok((i, Node::Element(doc)))
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
    fn integration1() -> Result<()> {
        let input = "bq. Don't suck the **brown stuff(tm)** off of \"2 pence\":http://royalmint.gov.uk coins; it ain't chocolate.";
        let (remaining_input, doc) = parcom(input)?;
        println!("🔎 {:?}", doc);
        assert_eq!("", remaining_input);
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
}
