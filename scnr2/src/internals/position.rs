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
