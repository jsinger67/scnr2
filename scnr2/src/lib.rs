//! A library for creating scanners in Rust.

// Re-export the scanner macro
pub use scnr2_macro::scanner;

// -------- Scanner Data Structures -------
// These structures are used to define the scanner's modes, tokens, and transitions.
// They are used in the generated code to encode the scanner data and behavior.
// ----------------------------------------

/// A scanner mode, which includes its name, transitions, and the DFA (Deterministic Finite
/// Automaton) that defines its behavior.
pub struct ScannerMode {
    pub name: &'static str,
    pub transitions: &'static [(usize, usize)],
    pub dfa: Dfa,
}
/// A Deterministic Finite Automaton (DFA) that consists of states.
pub struct Dfa {
    pub states: &'static [DfaState],
}
/// A state in the DFA, which includes transitions to other states and optional accept data.
pub struct DfaState {
    pub transitions: &'static [DfaTransition],
    pub accept_data: std::option::Option<AcceptData>,
}
/// Data associated with an accepting state in the DFA, including the type of token and lookahead
/// information.
pub struct AcceptData {
    pub token_type: usize,
    pub lookahead: Lookahead,
}
/// Lookahead information for the DFA, which can be positive or negative.
pub enum Lookahead {
    None,
    Positive(Dfa),
    Negative(Dfa),
}
/// A transition in the DFA, which includes a character class and the state to transition to.
pub struct DfaTransition {
    pub char_class: usize,
    pub to: usize,
}
