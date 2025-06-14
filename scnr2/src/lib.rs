//! A library for creating scanners in Rust.

// Re-export the scanner macro
pub use scnr2_macro::scanner;

// Expose only the necessary types and functions from the internals module
pub use crate::internals::ScannerImpl;

#[doc(hidden)]
pub mod internals;

// -------- Scanner Data Structures -------
// These structures are used to define the scanner's modes, tokens, and transitions.
// They are used in the generated code to encode the scanner data and behavior.
// ----------------------------------------

/// A scanner mode, which includes its name, transitions, and the DFA (Deterministic Finite
/// Automaton) that defines its behavior.
#[derive(Debug)]
pub struct ScannerMode {
    pub name: &'static str,
    pub transitions: &'static [(usize, usize)],
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
    pub transitions: &'static [DfaTransition],
    pub accept_data: std::option::Option<AcceptData>,
}
/// Data associated with an accepting state in the DFA, including the type of token and lookahead
/// information.
#[derive(Debug, Clone)]
pub struct AcceptData {
    pub token_type: usize,
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
pub struct Scanner {
    scanner_impl: ScannerImpl,
}

impl Scanner {
    /// Creates a new scanner with the initial mode and state.
    pub fn new(modes: &'static [ScannerMode]) -> Self {
        Scanner {
            scanner_impl: ScannerImpl {
                current_mode: 0,
                current_state: 0,
                modes,
            },
        }
    }
}
