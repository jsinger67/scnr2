//! CharIter struct and its implementation
use std::str::CharIndices;

use crate::internals::position::Position;

/// An iterator over characters in a string slice, yielding tuples of (char_index, char, Position).
/// The `CharIter` struct provides an iterator that tracks the position of characters in a string,
/// including their byte index, line number, and column number.
#[derive(Debug, Clone)]
pub struct CharIter<'a> {
    char_indices: CharIndices<'a>,
    offset: usize,
    line: usize,
    column: usize,
}

impl<'a> CharIter<'a> {
    /// Creates a new `CharIter` from the given string slice.
    pub fn new(haystack: &'a str, offset: usize) -> Self {
        let char_indices = if offset <= haystack.len() {
            haystack[offset..].char_indices()
        } else {
            haystack[haystack.len()..haystack.len()].char_indices()
        };
        CharIter {
            char_indices,
            offset: 0,
            line: 1,
            column: 0,
        }
    }

    /// Returns the current line and column numbers.
    pub fn position(&self) -> (usize, usize) {
        (self.line, self.column)
    }
}

// CharIter implements an iterator over characters in a string slice,
// yielding tuples of (char_index, char, Position) where Position
// contains the line and column numbers of the character.
impl Iterator for CharIter<'_> {
    type Item = (usize, char, Position); // (char_index, char, Position)

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((byte_index, ch)) = self.char_indices.next() {
            let (line, column) = if ch == '\n' {
                (self.line + 1, 0)
            } else {
                (self.line, self.column + 1)
            };
            self.offset = byte_index + ch.len_utf8();
            self.line = line;
            self.column = column;
            Some((byte_index, ch, Position::new(line, column)))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_iter() {
        let haystack = "Hello\nWorld";
        let mut iter = CharIter::new(haystack, 0);

        assert_eq!(iter.next(), Some((0, 'H', Position::new(1, 1))));
        assert_eq!(iter.next(), Some((1, 'e', Position::new(1, 2))));
        assert_eq!(iter.next(), Some((2, 'l', Position::new(1, 3))));
        assert_eq!(iter.next(), Some((3, 'l', Position::new(1, 4))));
        assert_eq!(iter.next(), Some((4, 'o', Position::new(1, 5))));
        assert_eq!(iter.next(), Some((5, '\n', Position::new(2, 0))));
        assert_eq!(iter.next(), Some((6, 'W', Position::new(2, 1))));
        assert_eq!(iter.next(), Some((7, 'o', Position::new(2, 2))));
        assert_eq!(iter.next(), Some((8, 'r', Position::new(2, 3))));
        assert_eq!(iter.next(), Some((9, 'l', Position::new(2, 4))));
        assert_eq!(iter.next(), Some((10, 'd', Position::new(2, 5))));
        assert_eq!(iter.next(), None);
    }
}
