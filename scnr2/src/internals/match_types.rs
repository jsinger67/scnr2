//! Module for match types used in the scanner.
use std::fmt::Display;

use crate::{Position, Span, internals::position::Positions};

/// A match in the input.
#[derive(Debug, Clone)]
/**
 * Represents a single token match in the input text.
 *
 * # Example
 * ```rust
 * use scnr2::{Match, Span};
 * let m = Match::new(3..7, 2);
 * assert_eq!(m.span, 3..7);
 * assert_eq!(m.token_type, 2);
 * ```
 *
 * The structure also optionally contains position information (line and column).
 */
pub struct Match {
    /// The position of the match in the input.
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

    /// Consumes the match, sets the positions and returns a new `Match` with the positions set.
    #[inline]
    pub fn with_positions(mut self, positions: Option<Positions>) -> Self {
        self.positions = positions;
        self
    }
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}..{}] tok {}{}",
            self.span.start,
            self.span.end,
            self.token_type,
            if let Some(pos) = self.positions.as_ref() {
                format!(" at {pos}")
            } else {
                String::new()
            }
        )
    }
}

/// Helper structure to manage the start of matches with their positions.
#[derive(Debug, Default)]
pub(crate) struct MatchStart {
    pub(crate) byte_index: usize,
    pub(crate) position: Option<Position>,
}

impl MatchStart {
    /// Creates a new `MatchStart` with the given byte index.
    pub(crate) fn new(byte_index: usize) -> Self {
        MatchStart {
            byte_index,
            position: None,
        }
    }

    /// Sets the position for the match start.
    pub(crate) fn with_position(mut self, position: Option<Position>) -> Self {
        self.position = position;
        self
    }
}

/// Helper structure to manage the end of matches with their positions, token type, and priority.
#[derive(Debug, Default)]
pub(crate) struct MatchEnd {
    pub(crate) byte_index: usize,
    pub(crate) position: Option<Position>,
    pub(crate) token_type: usize,
    pub(crate) priority: usize,
}

impl MatchEnd {
    /// Creates a new `MatchEnd` with the given byte index, token type, and priority.
    pub(crate) fn new(byte_index: usize, token_type: usize, priority: usize) -> Self {
        MatchEnd {
            byte_index,
            position: None,
            token_type,
            priority,
        }
    }

    /// Sets the position for the match end.
    pub(crate) fn with_position(mut self, position: Option<Position>) -> Self {
        self.position = position;
        self
    }
}
