/// A position in the haystack.
/// The position is represented by a line and column number.
/// The line and column numbers are 1-based.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// The line number in the haystack.
    pub line: usize,
    /// The column number in the haystack.
    pub column: usize,
}

impl Position {
    /// Creates a new `Position` with the given line and column numbers.
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }
}

impl Default for Position {
    /// Returns a default `Position` at line 1, column 1.
    fn default() -> Self {
        Position { line: 1, column: 1 }
    }
}

impl std::fmt::Display for Position {
    /// Formats the position as "line:column".
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// A match in the haystack.
#[derive(Debug, Clone, Default)]
pub struct Positions {
    /// The line and column number where the match started.
    pub start_position: Position,
    /// The line and column number where the match ended.
    pub end_position: Position,
}

impl Positions {
    /// Creates a new `Positions` with the given start and end positions.
    pub fn new(start_position: Position, end_position: Position) -> Self {
        Positions {
            start_position,
            end_position,
        }
    }
}
