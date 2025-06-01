#[cfg(test)]
#[macro_use]
extern crate rstest;

pub type Error = Box<dyn std::error::Error>;

/// The result type for the `scrn` crate.
pub type Result<T> = std::result::Result<T, crate::Error>;

/// The character_classes module contains the character class definitions
/// and utilities for SCNR2 generation.
mod character_classes;

/// The dfa module contains the DFA implementation.
mod dfa;

/// The codegen module contains the code generation logic for SCNR2.
pub mod generate;

/// The id module contains the ID types used in the SCNR2 generation.
mod ids;

/// Module that provides functions and types related to DFA minimization.
mod minimizer;

/// The nfa module contains the NFA implementation.
mod nfa;

/// The parser module contains the regex syntax parser.
mod parser;

/// The pattern module contains the pattern matching implementation.
mod pattern;

/// The code formatter module contains the logic to format Rust code.
mod rust_code_formatter;

/// The scanner data module.
mod scanner_data;

/// The scanner mode module contains the scanner mode's implementation.
mod scanner_mode;
