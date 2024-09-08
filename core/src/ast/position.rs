use std::cmp::Ordering;
use std::fmt;
use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub line: NonZeroU32,
    pub column: NonZeroU32,
}

impl From<(u32, u32)> for Position {
    fn from((line, column): (u32, u32)) -> Self {
        Self::new(line, column).unwrap()
    }
}

impl Position {
    pub fn new(line: u32, column: u32) -> Option<Self> {
        NonZeroU32::new(line)
            .and_then(|line| NonZeroU32::new(column).map(|column| Self { line, column }))
    }

    pub fn next_line(&self) -> Self {
        Self::new(self.line.get() + 1, 1).unwrap()
    }

    pub fn next_column(&self) -> Self {
        Self::new(self.line.get(), self.column.get() + 1).unwrap()
    }

    pub fn line(&self) -> u32 {
        self.line.get()
    }

    pub fn column(&self) -> u32 {
        self.column.get()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

mod tests {
    #[test]
    fn test_position_new() {
        use super::Position;
        assert_eq!(Position::new(0, 0), None);
        assert_eq!(Position::new(1, 0), None);
        assert_eq!(Position::new(0, 1), None);
        assert_eq!(Position::new(1, 1), Some((1, 1).into()));
    }

    #[test]
    fn test_position_cmp() {
        use super::Position;
        assert!(Position::from((1, 1)) < Position::from((1, 2)));
        assert!(Position::from((1, 1)) < Position::from((2, 1)));
    }

    #[test]
    fn test_position_eq() {
        use super::Position;
        assert_eq!(Position::from((1, 1)), Position::from((1, 1)));
        assert_ne!(Position::from((1, 1)), Position::from((1, 2)));
        assert_ne!(Position::from((1, 1)), Position::from((2, 1)));
    }

    #[test]
    fn get_line() {
        use super::Position;
        assert_eq!(Position::from((1, 1)).line(), 1);
        assert_eq!(Position::from((15, 1)).line(), 15);
    }

    #[test]
    fn get_column() {
        use super::Position;
        assert_eq!(Position::from((1, 1)).column(), 1);
        assert_eq!(Position::from((1, 15)).column(), 15);
    }

    #[test]
    fn next_line() {
        use super::Position;
        assert_eq!(Position::from((1, 1)).next_line(), Position::from((2, 1)));
        assert_eq!(Position::from((15, 1)).next_line(), Position::from((16, 1)));
    }

    #[test]
    fn next_column() {
        use super::Position;
        assert_eq!(Position::from((1, 1)).next_column(), Position::from((1, 2)));
        assert_eq!(
            Position::from((1, 15)).next_column(),
            Position::from((1, 16))
        );
    }
}
