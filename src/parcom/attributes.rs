use crate::parcom::Attributes;
use nom::IResult;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::combinator::opt;
use nom::multi::many0;

#[derive(Debug)]
enum AttrField {
    ClassesId(Vec<String>, Option<String>),
    Style(Option<String>),
}

fn classes_ids(i: &str) -> IResult<&str, AttrField> {
    let mut result_classes: Vec<String> = Vec::new();
    let mut result_id: Option<String> = None;
    let (i, _) = tag("(")(i)?;
    let (i, caught) = take_while1(|chr: char| !chr.is_control() && chr != ')')(i)?;
    let (i, _) = tag(")")(i)?;
    let (ids, classes) = opt(take_while(|chr: char| chr != '#'))(caught)?;
    for class in classes.unwrap_or("").split(" ") {
        if class == "" {
            continue;
        }
        result_classes.push(class.into());
    }
    for id in ids.split("#") {
        if id == "" {
            continue;
        }
        result_id = Some(id.into())
    }
    Ok((i, AttrField::ClassesId(result_classes, result_id)))
}

fn style(i: &str) -> IResult<&str, AttrField> {
    let (i, _) = tag("{")(i)?;
    let (i, caught) = take_while1(|chr: char| !chr.is_control() && chr != '}')(i)?;
    let (i, _) = tag("}")(i)?;
    Ok((i, AttrField::Style(Some(caught.into()))))
}

pub fn handle_attributes(i: &str) -> IResult<&str, Attributes> {
    let alts = (classes_ids, style);
    let mut attrs = Attributes::new();
    let (i, parts) = many0(alt(alts))(i)?;
    for part in parts {
        match part {
            AttrField::ClassesId(classes, id) => {
                attrs.classes = classes;
                attrs.id = id;
            }
            AttrField::Style(style) => attrs.style = style,
        }
    }
    Ok((i, attrs))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn handle_attributes_class() -> Result<()> {
        let (_, attrs) = handle_attributes("(orange mocha)")?;
        assert_eq!(attrs.classes, vec!("orange", "mocha"));
        assert_eq!(attrs.id, None);
        assert_eq!(attrs.style, None);
        Ok(())
    }

    #[test]
    fn handle_attributes_id() -> Result<()> {
        let (_, attrs) = handle_attributes("(#frap)")?;
        assert!(attrs.classes.is_empty());
        assert_eq!(attrs.id, Some("frap".into()));
        assert_eq!(attrs.style, None);
        Ok(())
    }

    #[test]
    fn handle_attributes_classes_id() -> Result<()> {
        let (_, attrs) = handle_attributes("(orange mocha #frap)")?;
        assert_eq!(attrs.classes, vec!("orange", "mocha"));
        assert_eq!(attrs.id, Some("frap".into()));
        assert_eq!(attrs.style, None);
        Ok(())
    }

    #[test]
    fn handle_attributes_styles() -> Result<()> {
        let (_, attrs) = handle_attributes("{color: orange}")?;
        assert!(attrs.classes.is_empty());
        assert_eq!(attrs.id, None);
        assert_eq!(attrs.style, Some("color: orange".into()));
        Ok(())
    }

    #[test]
    fn handle_attributes_styles_classes() -> Result<()> {
        let (_, attrs) = handle_attributes("{color: orange}(mocha)")?;
        assert_eq!(attrs.classes, vec!("mocha"));
        assert_eq!(attrs.id, None);
        assert_eq!(attrs.style, Some("color: orange".into()));
        Ok(())
    }

    #[test]
    fn handle_attributes_classes_styles() -> Result<()> {
        let (_, attrs) = handle_attributes("(mocha){color: orange}")?;
        assert_eq!(attrs.classes, vec!("mocha"));
        assert_eq!(attrs.id, None);
        assert_eq!(attrs.style, Some("color: orange".into()));
        Ok(())
    }
}
