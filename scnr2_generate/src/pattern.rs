//! A pattern as a data structure that is used during the construction of the NFA.
//! It contains the pattern string and the associated metadata.
//! Metadata includes the terminal type and a possibly empty lookahead constraint.
use crate::Result;

/// The lookahead constraint is used to ensure that the pattern matches only if it is followed by a
/// specific regex pattern. It also possible to demand that the pattern is not followed by a
/// specific regex pattern. In this case the lookahead is negative.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lookahead {
    /// A positive lookahead constraint that requires the pattern to be followed by a specific regex
    /// pattern.
    Positive(String),
    /// A negative lookahead constraint that requires the pattern to not be followed by a specific
    /// regex pattern.
    Negative(String),
}

impl Lookahead {
    /// Creates a new positive lookahead constraint with the given regex pattern.
    ///
    /// # Arguments
    /// * `pattern` - The regex pattern that must follow the main pattern.
    pub fn positive(pattern: String) -> Self {
        Lookahead::Positive(pattern)
    }

    /// Creates a new negative lookahead constraint with the given regex pattern.
    ///
    /// # Arguments
    /// * `pattern` - The regex pattern that must not follow the main pattern.
    pub fn negative(pattern: String) -> Self {
        Lookahead::Negative(pattern)
    }
}

/// A pattern is a data structure that is used during the construction of the NFA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    pub pattern: String,
    pub terminal_type: usize,
    pub lookahead: Option<Lookahead>,
}

impl Pattern {
    /// Creates a new pattern with the given pattern string, terminal type, and optional lookahead.
    ///
    /// # Arguments
    /// * `pattern` - The pattern string.
    /// * `terminal_type` - The terminal type associated with the pattern.
    pub fn new(pattern: String, terminal_type: usize) -> Self {
        Self {
            pattern,
            terminal_type,
            lookahead: None,
        }
    }

    /// Sets the lookahead constraint for the pattern.
    /// # Arguments
    /// * `lookahead` - The lookahead constraint to set.
    pub fn set_lookahead(&mut self, lookahead: Lookahead) {
        self.lookahead = Some(lookahead);
    }
}
