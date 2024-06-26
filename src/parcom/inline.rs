use crate::options::Symbol;
pub use crate::parcom::types::*;

use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_until, take_while1};
use nom::character::complete::char;
use nom::combinator::{all_consuming, fail};
use nom::multi::many1;
use nom::sequence::delimited;
use nom::IResult;

fn caps(i: &str) -> IResult<&str, Node> {
    let (i, matched) = take_while1(|chr: char| chr.is_uppercase())(i)?;
    if matched.len() <= 2 {
        return fail(i);
    }
    let element = Element::attrs_nodes(
        Tag::Span,
        Attributes::classes(vec!["caps".into()]),
        vec![Node::Plain(Plain::new(matched))],
    );
    Ok((i, Node::Element(element)))
}

fn plain(i: &str) -> IResult<&str, Node> {
    let (i, matched) = take_while1(|chr: char| chr.is_alphabetic())(i)?;
    Ok((i, Node::Plain(Plain::new(matched))))
}

fn word(i: &str) -> IResult<&str, Node> {
    alt((caps, plain))(i)
}

fn whitespace(i: &str) -> IResult<&str, Node> {
    let (i, matched) = take_while1(|chr: char| chr == ' ')(i)?;
    Ok((i, Node::Plain(Plain::new(matched))))
}

fn fallback1(i: &str) -> IResult<&str, Node> {
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
    let (_, nodes) = handle_inline(inside)?;
    let element = Element::nodes(Tag::Bold, nodes);
    Ok((i, Node::Element(element)))
}

fn strong(i: &str) -> IResult<&str, Node> {
    let (i, _) = char('*')(i)?;
    let (i, inside) = take_until("*")(i)?;
    let (i, _) = char('*')(i)?;
    let (_, nodes) = handle_inline(inside)?;
    let element = Element::nodes(Tag::Strong, nodes);
    Ok((i, Node::Element(element)))
}

fn italic(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("__")(i)?;
    let (i, inside) = take_until("__")(i)?;
    let (i, _) = tag("__")(i)?;
    let (_, nodes) = all_consuming(handle_inline)(inside)?;
    let element = Element::nodes(Tag::Italic, nodes);
    Ok((i, Node::Element(element)))
}

fn emphasized(i: &str) -> IResult<&str, Node> {
    let (i, _) = char('_')(i)?;
    let (i, inside) = take_until("_")(i)?;
    let (i, _) = char('_')(i)?;
    let (_, nodes) = all_consuming(handle_inline)(inside)?;
    let element = Element::nodes(Tag::Emphasis, nodes);
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
    let mut el = Element::new(Tag::Anchor);
    el.nodes = vec![Node::Plain(Plain::new(display))];
    el.attrs.href = Some(url.to_string());

    Ok((i, Node::Element(el)))
}

fn apostrophe(i: &str) -> IResult<&str, Node> {
    let (i, _) = char('\'')(i)?;
    Ok((i, Node::Symbol(Symbol::Apostrophe)))
}

fn trademark(i: &str) -> IResult<&str, Node> {
    let patterns = (tag("tm"), tag("TM"), tag("tM"), tag("Tm"));
    let (i, _) = alt(patterns)(i)?;
    Ok((i, Node::Symbol(Symbol::Trademark)))
}

fn registered(i: &str) -> IResult<&str, Node> {
    let patterns = (tag("r"), tag("R"));
    let (i, _) = alt(patterns)(i)?;
    Ok((i, Node::Symbol(Symbol::Registered)))
}

fn copyright(i: &str) -> IResult<&str, Node> {
    let patterns = (tag("c"), tag("C"));
    let (i, _) = alt(patterns)(i)?;
    Ok((i, Node::Symbol(Symbol::Copyright)))
}

fn half(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("1/2")(i)?;
    Ok((i, Node::Symbol(Symbol::Half)))
}

fn quarter(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("1/4")(i)?;
    Ok((i, Node::Symbol(Symbol::Quarter)))
}

fn three_quarters(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("3/4")(i)?;
    Ok((i, Node::Symbol(Symbol::ThreeQuarters)))
}

fn degrees(i: &str) -> IResult<&str, Node> {
    let patterns = (tag("o"), tag("O"));
    let (i, _) = alt(patterns)(i)?;
    Ok((i, Node::Symbol(Symbol::Degrees)))
}

fn plus_minus(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("+/-")(i)?;
    Ok((i, Node::Symbol(Symbol::PlusMinus)))
}

fn ellipsis(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("...")(i)?;
    Ok((i, Node::Symbol(Symbol::Ellipsis)))
}

fn simple_symbols(i: &str) -> IResult<&str, Node> {
    let symbols = (
        trademark,
        registered,
        copyright,
        half,
        quarter,
        three_quarters,
        degrees,
        plus_minus,
    );
    let with_parens = delimited(char('('), alt(symbols), char(')'));
    let with_squares = delimited(char('['), alt(symbols), char(']'));
    alt((with_parens, with_squares))(i)
}

fn footnote_ref(i: &str) -> IResult<&str, Node> {
    let (i, _) = char('[')(i)?;
    let (i, matched) = take_while1(|chr: char| chr.is_ascii_digit())(i)?;
    let (i, superscript) = if i.starts_with("!]") {
        (&i[2..], Node::Plain(Plain::new(matched)))
    } else if i.starts_with("]") {
        let link_down = Element::attrs_nodes(
            Tag::Anchor,
            Attributes::href("#fn".into()),
            vec![Node::Plain(Plain::new(matched))],
        );
        (&i[1..], link_down.into())
    } else {
        return fail(i);
    };
    let element = Element::attrs_nodes(
        Tag::FootnoteRef,
        Attributes::classes_id(vec!["footnote".into()], "fnrev".into()),
        vec![superscript],
    );
    Ok((i, Node::Element(element)))
}

fn emdash(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag("--")(i)?;
    Ok((i, Node::Symbol(Symbol::Emdash)))
}

/// This needs to be parsed before `whitespace`
fn endash(i: &str) -> IResult<&str, Node> {
    let (i, _) = tag(" - ")(i)?;
    Ok((i, Node::Symbol(Symbol::Endash)))
}

pub fn handle_inline(i: &str) -> IResult<&str, Vec<Node>> {
    let alts = (
        word,
        bold,
        strong,
        italic,
        emphasized,
        endash, // endash must be before whitespace
        whitespace,
        footnote_ref,
        apostrophe,
        ellipsis,
        emdash,
        simple_symbols,
        link,
        newline,
        fallback1,
    );
    all_consuming(many1(alt(alts)))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn handle_inline1() -> Result<()> {
        let input = "they're in the computer";
        let (remaining, node) = handle_inline(input)?;
        println!("🔎 {:?}", node);
        assert_eq!(remaining, "");
        Ok(())
    }

    #[test]
    fn handle_inline2() -> Result<()> {
        let input = "*hi* **hello** *hi*";
        let (remaining, node) = handle_inline(input)?;
        println!("🔎 {:?}", node);
        assert_eq!(remaining, "");
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
