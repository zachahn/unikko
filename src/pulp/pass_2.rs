use super::pass_1::{FirstPass, FirstPassEvent};

#[derive(Debug)]
pub enum SecondPassEvent {
    Paragraph,
    ParagraphEnd,

    Text(usize, usize),
    LineBreak(usize, usize),
}

pub struct SecondPass<'a> {
    input: &'a str,
    len: usize,
    first_pass: FirstPass<'a>,
    current_1_event: Option<FirstPassEvent>,
    position: usize,
    previous: Option<SecondPassEvent>,
    stack: Vec<SecondPassEvent>,
    expecting_block: bool,
}

impl<'a> SecondPass<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input,
            len: input.len(),
            first_pass: FirstPass::new(input),
            current_1_event: None,
            position: 0,
            previous: None,
            stack: vec![],
            expecting_block: true,
        }
    }
}

impl<'a> Iterator for SecondPass<'a> {
    type Item = SecondPassEvent;

    fn next(&mut self) -> Option<Self::Item> {
        // Pull the next FirstPassNode
        if self.current_1_event.is_none() {
            self.current_1_event = self.first_pass.next();
            match self.current_1_event {
                None => self.position = self.len,
                Some(FirstPassEvent::Line(start, _)) => self.position = start,
                Some(FirstPassEvent::Break(start, _)) => self.position = start,
                Some(FirstPassEvent::NewLine(start, _)) => self.position = start,
                Some(FirstPassEvent::Error(start, _)) => self.position = start,
            }
        }

        // Handle the end of the document
        if self.current_1_event.is_none() && !self.stack.is_empty() {
            self.current_1_event = Some(FirstPassEvent::Break(self.len, self.len))
        }

        match self.current_1_event {
            None => return None,
            Some(FirstPassEvent::Line(start, end)) => {
                if self.expecting_block {
                    self.expecting_block = false;
                    self.stack.push(SecondPassEvent::Paragraph);
                    return Some(SecondPassEvent::Paragraph);
                }

                self.current_1_event = None;
                return Some(SecondPassEvent::Text(start, end));
            }
            Some(FirstPassEvent::Break(_, _)) => {
                self.current_1_event = None;
                self.expecting_block = true;
                match self.stack.pop() {
                    None => return self.next(),
                    Some(SecondPassEvent::Paragraph) => return Some(SecondPassEvent::ParagraphEnd),
                    Some(SecondPassEvent::ParagraphEnd) => return self.next(),
                    Some(SecondPassEvent::Text(_, _)) => return self.next(),
                    Some(SecondPassEvent::LineBreak(_, _)) => return self.next(),
                }
            }
            Some(FirstPassEvent::NewLine(start, end)) => {
                self.current_1_event = None;
                return Some(SecondPassEvent::LineBreak(start, end));
            }
            Some(FirstPassEvent::Error(_, _)) => {
                self.current_1_event = None;
                return self.next();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn implicit_paragraph() -> Result<()> {
        let mut pulp = SecondPass::new("A paragraph");
        assert!(matches!(pulp.next(), Some(SecondPassEvent::Paragraph)));
        assert!(matches!(pulp.next(), Some(SecondPassEvent::Text(0, 11))));
        assert!(matches!(pulp.next(), Some(SecondPassEvent::ParagraphEnd)));
        assert!(matches!(pulp.next(), None));
        Ok(())
    }

    #[test]
    fn implicit_paragraphs() -> Result<()> {
        let mut pulp = SecondPass::new("Paragraph 1\n\nParagraph 2");
        assert!(matches!(pulp.next(), Some(SecondPassEvent::Paragraph)));
        assert!(matches!(pulp.next(), Some(SecondPassEvent::Text(0, 11))));
        assert!(matches!(pulp.next(), Some(SecondPassEvent::ParagraphEnd)));
        assert!(matches!(pulp.next(), Some(SecondPassEvent::Paragraph)));
        assert!(matches!(pulp.next(), Some(SecondPassEvent::Text(13, 24))));
        assert!(matches!(pulp.next(), Some(SecondPassEvent::ParagraphEnd)));
        assert!(matches!(pulp.next(), None));
        Ok(())
    }

    #[test]
    fn paragraph_and_not_a_paragraph() -> Result<()> {
        let mut pulp = SecondPass::new("Paragraph and newline\np. with fake");
        assert!(matches!(pulp.next(), Some(SecondPassEvent::Paragraph)));
        assert!(matches!(pulp.next(), Some(SecondPassEvent::Text(0, 21))));
        assert!(matches!(
            pulp.next(),
            Some(SecondPassEvent::LineBreak(21, 22))
        ));
        assert!(matches!(pulp.next(), Some(SecondPassEvent::Text(22, 34))));
        assert!(matches!(pulp.next(), Some(SecondPassEvent::ParagraphEnd)));
        assert!(matches!(pulp.next(), None));
        Ok(())
    }
}
