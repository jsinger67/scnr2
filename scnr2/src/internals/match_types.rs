use crate::{Span, internals::position::Position};

/// A match in the haystack.
#[derive(Debug, Clone)]
pub struct Match {
    /// The position of the match in the haystack.
    pub span: Span,
    /// The type of token matched.
    pub token_type: usize,
}

impl Match {
    /// Creates a new `Match` from the given span and token type.
    pub fn new(span: Span, token_type: usize) -> Self {
        Match { span, token_type }
    }
}

/// A match with additional line and column information.
#[derive(Debug, Clone)]
pub struct MatchWithPosition {
    /// The position of the match in the haystack.
    pub span: Span,
    /// The type of token matched.
    pub token_type: usize,
    /// The line and column number where the match started.
    pub start_position: Position,
    /// The line and column number where the match ended.
    pub end_position: Position,
}

impl MatchWithPosition {
    /// Creates a new `MatchWithPosition` from a `Match` and the line and column information.
    pub fn new(m: Match, start_position: Position, end_position: Position) -> Self {
        MatchWithPosition {
            span: m.span,
            token_type: m.token_type,
            start_position,
            end_position,
        }
    }
}
