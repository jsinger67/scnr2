pub type Error = Box<dyn std::error::Error>;

/// The result type for the `scrn` crate.
pub type Result<T> = std::result::Result<T, crate::Error>;

/// The nfa module contains the NFA implementation.
mod nfa;

/// The parser module contains the regex syntax parser.
mod parser;
