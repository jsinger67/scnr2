use crate::{
    Span,
    internals::position::{Position, Positions},
};

/// A match in the haystack.
#[derive(Debug, Clone)]
pub struct Match {
    /// The position of the match in the haystack.
    pub span: Span,
    /// The type of token matched.
    pub token_type: usize,
    /// The positions of the match in terms of line and column numbers.
    pub positions: Option<Positions>,
}

impl Match {
    /// Creates a new `Match` from the given span and token type.
    pub fn new(span: Span, token_type: usize) -> Self {
        Match {
            span,
            token_type,
            positions: None,
        }
    }

    /// Sets the positions of the match.
    #[inline]
    pub fn set_positions(&mut self, start_position: Position, end_position: Position) {
        self.positions = Some(Positions {
            start_position,
            end_position,
        });
    }

    /// Consumes the match, sets the positions and returns a new `Match` with the positions set.
    #[inline]
    pub fn with_positions(mut self, start_position: Position, end_position: Position) -> Self {
        self.set_positions(start_position, end_position);
        self
    }
}
