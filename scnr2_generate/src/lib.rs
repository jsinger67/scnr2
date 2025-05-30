#[cfg(test)]
#[macro_use]
extern crate rstest;

pub type Error = Box<dyn std::error::Error>;

/// The result type for the `scrn` crate.
pub type Result<T> = std::result::Result<T, crate::Error>;

/// The character_classes module contains the character class definitions
/// and utilities for SCNR2 generation.
mod character_classes;

/// The nfa module contains the NFA implementation.
mod nfa;

/// The parser module contains the regex syntax parser.
mod parser;

/// The pattern module contains the pattern matching implementation.
mod pattern;
