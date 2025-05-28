/// The result type for the `scrn` crate.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The parser module contains the regex syntax parser.
mod parser;
pub(crate) use parser::parse_regex;
