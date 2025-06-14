//! Internals of the scanner module.

use std::str::CharIndices;

/// Scanner implementation details.
#[derive(Debug, Clone)]
pub struct ScannerImpl {
    pub(crate) current_mode: usize,
    pub(crate) current_state: usize,
    pub(crate) modes: &'static [crate::ScannerMode],
}

impl ScannerImpl {
    /// Creates a new scanner implementation with the given modes.
    pub fn new(modes: &'static [crate::ScannerMode]) -> Self {
        ScannerImpl {
            current_mode: 0,
            current_state: 0,
            modes,
        }
    }
}

/// A struct that provides the iterator that finds matches in a string slice
#[derive(Debug, Clone)]
struct FindMatches<'a> {
    haystack: &'a str,
    char_iter: CharIter<'a>,
    offset: usize,
}

impl<'a> FindMatches<'a> {
    /// Creates a new `FindIter` from the given string slice and start position.
    pub fn new(haystack: &'a str, offset: usize) -> Self {
        FindMatches {
            haystack,
            char_iter: CharIter::new(haystack, offset),
            offset,
        }
    }
}

/// A character iterator that provides the current line and column numbers along with the character
/// indices.
#[derive(Debug, Clone)]
struct CharIter<'a> {
    char_indices: CharIndices<'a>,
    offset: usize,
    line: usize,
    column: usize,
}

impl<'a> CharIter<'a> {
    /// Creates a new `CharIter` from the given string slice.
    pub fn new(haystack: &'a str, offset: usize) -> Self {
        let char_indices = if offset <= haystack.len() {
            // Split the input a byte position `offset` and create a new char_indices iterator.
            haystack[offset..].char_indices()
        } else {
            // The position is greater than the length of the haystack.
            // Take an empty slice after the haystack to create an empty char_indices iterator.
            haystack[haystack.len()..haystack.len()].char_indices()
        };
        CharIter {
            char_indices,
            offset: 0,
            line: 1,
            column: 1,
        }
    }

    /// Returns the current line and column numbers.
    pub fn position(&self) -> (usize, usize) {
        (self.line, self.column)
    }
}

impl Iterator for CharIter<'_> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((byte_index, ch)) = self.char_indices.next() {
            let char_index = byte_index - self.offset;
            let (line, column) = if ch == '\n' {
                (self.line + 1, 0)
            } else {
                (self.line, self.column + 1)
            };
            self.offset = byte_index + ch.len_utf8();
            self.line = line;
            self.column = column;
            Some((char_index, ch))
        } else {
            None
        }
    }
}
