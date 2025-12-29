#[derive(Debug)]
pub enum FirstPassEvent {
    Line(usize, usize),
    Break(usize, usize),
    NewLine(usize, usize),
    Error(usize, usize),
}

pub struct FirstPass<'a> {
    input: &'a [u8],
    len: usize,
    position: usize,
}

impl<'a> FirstPass<'a> {
    pub fn new(input: &'a str) -> Self {
        let bytes = input.as_bytes();
        Self {
            input: bytes,
            len: bytes.len(),
            position: 0,
        }
    }

    fn peek(&self, delta: usize) -> u8 {
        let pos = self.position + delta;

        if pos < self.len {
            self.input[pos]
        } else {
            u8::MIN
        }
    }
}

impl<'a> Iterator for FirstPass<'a> {
    type Item = FirstPassEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.len {
            return None;
        }

        let start = self.position;
        let mut delta: usize = 0;
        let mut starting_newline = false;

        loop {
            let char = self.peek(delta);
            match char {
                b'\0' => break,
                b'\n' => {
                    if delta == 0 {
                        starting_newline = true;
                    } else if !starting_newline {
                        break;
                    }
                }
                _ => {
                    if starting_newline {
                        break;
                    }
                }
            }

            delta += 1
        }

        self.position = start + delta;

        if starting_newline {
            match delta {
                0 => Some(FirstPassEvent::Error(start, start + delta)),
                1 => Some(FirstPassEvent::NewLine(start, start + delta)),
                _ => Some(FirstPassEvent::Break(start, start + delta)),
            }
        } else {
            Some(FirstPassEvent::Line(start, start + delta))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn implicit_paragraph() -> Result<()> {
        let mut pulp = FirstPass::new("A paragraph.\n\nAnd a paragraph with\na line break.");
        assert!(matches!(pulp.next(), Some(FirstPassEvent::Line(0, 12))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::Break(12, 14))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::Line(14, 34))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::NewLine(34, 35))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::Line(35, 48))));
        assert!(matches!(pulp.next(), None));
        Ok(())
    }

    #[test]
    fn bulleted_list() -> Result<()> {
        let mut pulp = FirstPass::new("* Item A\n** Item A1\n** Item A2\nItem B\nItem C\n");
        assert!(matches!(pulp.next(), Some(FirstPassEvent::Line(0, 8))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::NewLine(8, 9))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::Line(9, 19))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::NewLine(19, 20))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::Line(20, 30))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::NewLine(30, 31))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::Line(31, 37))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::NewLine(37, 38))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::Line(38, 44))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::NewLine(44, 45))));
        assert!(matches!(pulp.next(), None));
        Ok(())
    }

    #[test]
    fn starting_blank_line() -> Result<()> {
        let mut pulp = FirstPass::new("\nHello");
        assert!(matches!(pulp.next(), Some(FirstPassEvent::NewLine(0, 1))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::Line(1, 6))));
        Ok(())
    }

    #[test]
    fn starting_blank_lines() -> Result<()> {
        let mut pulp = FirstPass::new("\n\nHello");
        assert!(matches!(pulp.next(), Some(FirstPassEvent::Break(0, 2))));
        assert!(matches!(pulp.next(), Some(FirstPassEvent::Line(2, 7))));
        Ok(())
    }
}
