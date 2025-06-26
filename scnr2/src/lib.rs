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
    char_iter::CharIterWithPosition,
    find_matches::{FindMatches, FindMatchesWithPosition},
    match_types::Match,
    position::Position,
    scanner_impl::ScannerImpl,
};

// -------- Scanner Data Structures -------
// These structures are used to define the scanner's modes, tokens and transitions.
// They are used in the generated code to encode the scanner data and behavior.
// ----------------------------------------

/// A range type representing a span in the source code, typically used for token match positions.
pub type Span = core::ops::Range<usize>;

/// A transition in the scanner
#[derive(Debug, Clone)]
pub enum Transition {
    /// A transition to a new scanner mode triggered by a token type number.
    /// The first element is the token type number, and the second element is the new scanner mode
    /// index.
    /// This transition is used to set the current scanner mode.
    SetMode(usize, usize),
    /// A transition to a new scanner mode triggered by a token type number.
    /// The first element is the token type number, and the second element is the new scanner mode
    /// index.
    /// This transition is used to push the current mode on the mode stack to be able to return to
    /// it later.
    PushMode(usize, usize),
    /// A transition back to a formerly pushed scanner mode triggered by a token type number.
    /// This transition is used to pop the current scanner mode from the stack.
    /// If the mode stack is empty, it stays in the current mode.
    PopMode(usize),
}

impl Transition {
    /// Returns the token type number of this transition.
    pub fn token_type(&self) -> usize {
        match self {
            Transition::SetMode(token_type, _)
            | Transition::PushMode(token_type, _)
            | Transition::PopMode(token_type) => *token_type,
        }
    }
}

/// A scanner mode, which includes its name, transitions, and the DFA (Deterministic Finite
/// Automaton) that defines its behavior.
#[derive(Debug)]
pub struct ScannerMode {
    pub name: &'static str,
    pub transitions: &'static [Transition],
    pub dfa: Dfa,
}

/// A Deterministic Finite Automaton (DFA) that consists of states.
#[derive(Debug, Clone)]
pub struct Dfa {
    pub states: &'static [DfaState],
}
/// A state in the DFA, which includes transitions to other states and optional accept data.
#[derive(Debug, Clone)]
pub struct DfaState {
    /// The transitions for this state indexed by character class index.
    /// Each transition is an `Option<DfaTransition>`, where `None` indicates no
    /// transition for that character class.
    pub transitions: &'static [Option<DfaTransition>],
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
    pub to: usize,
}
