use crate::ast::position::Position;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    start: Position,
    end: Position,
}

impl Span {
    #[inline]
    #[track_caller]
    #[must_use]
    pub fn new(start: Position, end: Position) -> Self {
        assert!(start <= end, "a span cannot start after its end");

        Self { start, end }
    }

    #[inline]
    #[must_use]
    pub const fn start(self) -> Position {
        self.start
    }

    #[inline]
    #[must_use]
    pub const fn end(self) -> Position {
        self.end
    }

    pub fn contains<S>(self, other: S) -> bool
    where
        S: Into<Self>,
    {
        let other = other.into();
        self.start <= other.start && self.end >= other.end
    }
}

impl From<Position> for Span {
    fn from(pos: Position) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.end < other.start {
            Some(Ordering::Less)
        } else if self.start > other.end {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}..{}]", self.start, self.end)
    }
}
