//! # SCNR2 Code Generation Library
//!
//! This crate provides compile-time code generation for creating efficient
//! lexical scanners. It converts regex patterns into optimized DFA-based
//! scanners that can be used at runtime.
//!
//! ## Example Usage
//! ```ignore
//! use scnr2_macro::scanner;
//!
//! scanner! {
//!     MyScanner {
//!         mode INITIAL {
//!             token r"\d+" => NUMBER;
//!             token r"[a-zA-Z_]\w*" => IDENTIFIER;
//!         }
//!     }
//! }
//! ```

#[cfg(test)]
#[macro_use]
extern crate rstest;

pub type Error = Box<dyn std::error::Error>;

/// The result type for the `scrn` crate.
pub type Result<T> = std::result::Result<T, crate::Error>;

/// The character_classes module contains the character class definitions
/// and utilities for SCNR2 generation.
pub mod character_classes;

/// The dfa module contains the DFA implementation.
pub mod dfa;

/// The codegen module contains the code generation logic for SCNR2.
pub mod generate;

/// The id module contains the ID types used in the SCNR2 generation.
pub mod ids;

/// Module that provides functions and types related to DFA minimization.
pub mod minimizer;

/// The nfa module contains the NFA implementation.
pub mod nfa;

/// The pattern module contains the pattern matching implementation.
pub mod pattern;

/// The scanner data module.
pub mod scanner_data;

/// The scanner mode module contains the scanner mode's implementation.
pub mod scanner_mode;
