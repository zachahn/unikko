#[derive(Debug)]
pub enum FirstPassNode {
    Text(usize, usize),
    Break(usize, usize),
    NewLine(usize, usize),
    Huh(usize, usize),
}

pub struct PulpFirstPass<'a> {
    input: &'a [u8],
    len: usize,
    position: usize,
}

impl<'a> PulpFirstPass<'a> {
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

impl<'a> Iterator for PulpFirstPass<'a> {
    type Item = FirstPassNode;

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
                0 => Some(FirstPassNode::Huh(start, start + delta)),
                1 => Some(FirstPassNode::NewLine(start, start + delta)),
                _ => Some(FirstPassNode::Break(start, start + delta)),
            }
        } else {
            Some(FirstPassNode::Text(start, start + delta))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn implicit_paragraph() -> Result<()> {
        let mut pulp = PulpFirstPass::new("A paragraph.\n\nAnd a paragraph with\na line break.");
        assert!(matches!(pulp.next(), Some(FirstPassNode::Text(0, 12))));
        assert!(matches!(pulp.next(), Some(FirstPassNode::Break(12, 14))));
        assert!(matches!(pulp.next(), Some(FirstPassNode::Text(14, 34))));
        assert!(matches!(pulp.next(), Some(FirstPassNode::NewLine(34, 35))));
        assert!(matches!(pulp.next(), Some(FirstPassNode::Text(35, 48))));
        assert!(matches!(pulp.next(), None));
        Ok(())
    }

    #[test]
    fn bulleted_list() -> Result<()> {
        let mut pulp = PulpFirstPass::new("* Item A\n** Item A1\n** Item A2\nItem B\nItem C\n");
        assert!(matches!(pulp.next(), Some(FirstPassNode::Text(0, 8))));
        assert!(matches!(pulp.next(), Some(FirstPassNode::NewLine(8, 9))));
        assert!(matches!(pulp.next(), Some(FirstPassNode::Text(9, 19))));
        assert!(matches!(pulp.next(), Some(FirstPassNode::NewLine(19, 20))));
        assert!(matches!(pulp.next(), Some(FirstPassNode::Text(20, 30))));
        assert!(matches!(pulp.next(), Some(FirstPassNode::NewLine(30, 31))));
        assert!(matches!(pulp.next(), Some(FirstPassNode::Text(31, 37))));
        assert!(matches!(pulp.next(), Some(FirstPassNode::NewLine(37, 38))));
        assert!(matches!(pulp.next(), Some(FirstPassNode::Text(38, 44))));
        assert!(matches!(pulp.next(), Some(FirstPassNode::NewLine(44, 45))));
        assert!(matches!(pulp.next(), None));
        Ok(())
    }
}
