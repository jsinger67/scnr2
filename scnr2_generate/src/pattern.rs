//! A pattern as a data structure that is used during the construction of the NFA.
//! It contains the pattern string and the associated metadata.
//! Metadata includes the terminal type and a possibly empty lookahead constraint.
use crate::{Result, nfa::Nfa};

/// The lookahead constraint is used to ensure that the pattern matches only if it is followed by a
/// specific regex pattern, a so called positive lookahead. It is also possible to demand that the
/// pattern is not followed by a specific regex pattern. In this case the lookahead is negative.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Lookahead {
    /// No lookahead constraint is applied. This is used when no specific lookahead is required.
    #[default]
    None,
    /// A positive lookahead constraint that requires the pattern to be followed by a specific regex
    /// pattern.
    Positive(Nfa),
    /// A negative lookahead constraint that requires the pattern to not be followed by a specific
    /// regex pattern.
    Negative(Nfa),
}

impl Lookahead {
    /// Creates a new positive lookahead constraint with the given regex pattern.
    ///
    /// # Arguments
    /// * `pattern` - The regex pattern that must follow the main pattern.
    pub fn positive(pattern: String) -> Result<Self> {
        // Convert the string pattern into an NFA.
        // The `usize::MAX` is used to indicate that the pattern has no associated terminal type.
        let nfa = Nfa::build(&Pattern::new(pattern, usize::MAX))
            .map_err(|e| format!("Failed to create NFA from regex pattern: {}", e))?;
        Ok(Lookahead::Positive(nfa))
    }

    /// Creates a new negative lookahead constraint with the given regex pattern.
    ///
    /// # Arguments
    /// * `pattern` - The regex pattern that must not follow the main pattern.
    pub fn negative(pattern: String) -> Result<Self> {
        // Convert the string pattern into an NFA.
        // The `usize::MAX` is used to indicate that the pattern has no associated terminal type.
        let nfa = Nfa::build(&Pattern::new(pattern, usize::MAX))
            .map_err(|e| format!("Failed to create NFA from regex pattern: {}", e))?;
        Ok(Lookahead::Negative(nfa))
    }

    // /// Checks if the lookahead is empty, meaning it has no constraints.
    // pub fn is_empty(&self) -> bool {
    //     matches!(self, Lookahead::None)
    // }

    // /// Checks if the lookahead is positive, meaning it has a positive lookahead constraint.
    // /// Returns `true` if the lookahead is positive, `false` otherwise.
    // pub fn is_positive(&self) -> bool {
    //     matches!(self, Lookahead::Positive(_))
    // }

    // /// Checks if the lookahead is negative, meaning it has a negative lookahead constraint.
    // /// Returns `true` if the lookahead is negative, `false` otherwise.
    // pub fn is_negative(&self) -> bool {
    //     matches!(self, Lookahead::Negative(_))
    // }
}

/// A pattern is a data structure that is used during the construction of the NFA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    pub pattern: String,
    pub terminal_type: usize,
    pub lookahead: Lookahead,
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
            lookahead: Lookahead::None,
        }
    }

    /// Sets the lookahead constraint for the pattern while consuming the current pattern.
    /// # Arguments
    /// * `lookahead` - The lookahead constraint to set.
    pub fn with_lookahead(mut self, lookahead: Lookahead) -> Self {
        self.lookahead = lookahead;
        self
    }

    /// Sets the lookahead constraint for the pattern.
    /// # Arguments
    /// * `lookahead` - The lookahead constraint to set.
    pub fn set_lookahead(&mut self, lookahead: Lookahead) {
        self.lookahead = lookahead;
    }
}
