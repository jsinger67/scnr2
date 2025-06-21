//! A library for creating scanners in Rust.
//! This library provides a macro to define scanners and their modes, tokens, and transitions.
//! It also includes data structures for implementing a scanner runtime, including a
//! scanner implementation, DFA (Deterministic Finite Automaton) states, and transitions.

// Re-export the scanner macro
pub use scnr2_macro::scanner;

// Expose only some necessary types and functions from the internals module
#[doc(hidden)]
pub mod internals;
pub use crate::internals::{
    char_iter::CharIter,
    find_matches::{FindMatches, FindMatchesWithPosition},
    match_types::{Match, MatchWithPosition},
    position::Position,
    scanner_impl::ScannerImpl,
};

// -------- Scanner Data Structures -------
// These structures are used to define the scanner's modes, tokens, and transitions.
// They are used in the generated code to encode the scanner data and behavior.
// ----------------------------------------

/// A range type representing a span in the source code, typically used for token match positions.
pub type Span = core::ops::Range<usize>;

/// A scanner mode, which includes its name, transitions, and the DFA (Deterministic Finite
/// Automaton) that defines its behavior.
#[derive(Debug)]
pub struct ScannerMode {
    pub name: &'static str,
    pub transitions: &'static [(usize, usize)],
    pub dfa: Dfa,
}

impl ScannerMode {
    /// Returns the index of the new mode based on the token type.
    /// If no transition exists for the token type, it returns `None`.
    pub fn next_mode(&self, token_type: usize) -> Option<usize> {
        self.transitions
            .iter()
            .find(|&&(t, _)| t == token_type)
            .map(|&(_, next_mode)| next_mode)
    }
}

/// A Deterministic Finite Automaton (DFA) that consists of states.
#[derive(Debug, Clone)]
pub struct Dfa {
    pub states: &'static [DfaState],
}
/// A state in the DFA, which includes transitions to other states and optional accept data.
#[derive(Debug, Clone)]
pub struct DfaState {
    pub transitions: &'static [DfaTransition],
    pub accept_data: std::option::Option<AcceptData>,
}
/// Data associated with an accepting state in the DFA, including the type of token and lookahead
/// information.
#[derive(Debug, Clone)]
pub struct AcceptData {
    pub token_type: usize,
    pub priority: usize,
    pub lookahead: Lookahead,
}
/// Lookahead information for the DFA, which can be positive or negative.
#[derive(Debug, Clone)]
pub enum Lookahead {
    None,
    Positive(Dfa),
    Negative(Dfa),
}
/// A transition in the DFA, which includes a character class and the state to transition to.
#[derive(Debug, Clone)]
pub struct DfaTransition {
    pub char_class: usize,
    pub to: usize,
}

/// A scanner that can be used to match tokens against the defined modes and transitions.
pub struct Scanner<F>
where
    F: Fn(char) -> Option<usize> + 'static,
{
    scanner_impl: ScannerImpl,
    match_function: &'static F,
}

impl<F> Scanner<F>
where
    F: Fn(char) -> Option<usize> + 'static,
{
    /// Creates a new scanner with the initial mode and state.
    pub fn new(modes: &'static [ScannerMode], match_function: &'static F) -> Self {
        Scanner {
            scanner_impl: ScannerImpl::new(modes),
            match_function,
        }
    }

    /// Creates a new `FindMatches` iterator for the given haystack and offset.
    pub fn find_matches<'a>(
        &'a self,
        haystack: &'a str,
        offset: usize,
    ) -> crate::internals::find_matches::FindMatches<'a, F> {
        self.scanner_impl
            .find_matches(haystack, offset, self.match_function)
    }

    /// Creates a new `FindMatchesWithPosition` iterator for the given haystack and offset.
    pub fn find_matches_with_position<'a>(
        &'a self,
        haystack: &'a str,
        offset: usize,
    ) -> crate::internals::find_matches::FindMatchesWithPosition<'a, F> {
        self.scanner_impl
            .find_matches_with_position(haystack, offset, self.match_function)
    }
}
