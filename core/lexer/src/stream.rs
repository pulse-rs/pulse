use pulse_ast::position::Position;
use std::io;
use std::io::Error;

#[derive(Debug)]
pub(super) struct TokenStream {
    pos: Position,
    peeked: [Option<u32>; 4],
    chars: Vec<u32>,
}

impl TokenStream {
    pub(super) const fn pos(&self) -> Position {
        self.pos
    }
}

impl TokenStream {
    pub(super) fn new(input: &str) -> Self {
        Self {
            pos: Position::new(1, 1).unwrap(),
            peeked: [None; 4],
            chars: input.chars().map(|c| c as u32).collect(),
        }
    }

    pub fn peek_char(&mut self) -> Result<Option<u32>, Error> {
        if let Some(c) = self.peeked[0] {
            return Ok(Some(c));
        }

        let next = self.chars.get(0).copied();
        self.peeked[0] = next;
        Ok(next)
    }

    pub(crate) fn next_char(&mut self) -> io::Result<Option<u32>> {
        let ch = if let Some(c) = self.peeked[0] {
            self.peeked[0] = None;
            self.peeked.rotate_left(1);
            Some(c)
        } else {
            self.chars.get(0).copied()
        };

        if let Some(_) = ch {
            self.chars.remove(0);
        }

        Ok(ch)
    }
}
