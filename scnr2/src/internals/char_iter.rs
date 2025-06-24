//! CharIter structs and their implementations
use std::str::CharIndices;

use crate::internals::position::Position;

/// Represents a character item with its index, character, and position in the string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharItem {
    pub byte_index: usize,
    pub ch: char,
    pub position: Option<Position>,
}

impl CharItem {
    /// Creates a new `CharItem` with the given character index, character, and position.
    #[inline]
    pub fn new(char_index: usize, ch: char) -> Self {
        CharItem {
            byte_index: char_index,
            ch,
            position: None,
        }
    }
    /// Consumes the `CharItem` and returns a new `CharItem` with the position set.
    #[inline]
    pub fn with_position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }
}

/// An iterator over characters in a string slice, yielding tuples of (char_index, char, Position).
/// The `CharIter` struct provides an iterator that tracks the position of characters in a string,
/// including their byte index, line number, and column number.
#[derive(Debug, Clone)]
pub struct CharIter<'a> {
    char_indices: CharIndices<'a>,
    offset: usize,
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
        }
    }

    /// Returns the next character without advancing the iterator.
    pub(crate) fn peek(&mut self) -> Option<CharItem> {
        if let Some((byte_index, ch)) = self.char_indices.clone().next() {
            Some(CharItem::new(byte_index, ch))
        } else {
            None
        }
    }
}

// CharIter implements an iterator over characters in a string slice,
// yielding tuples of (char_index, char) where Position
// contains the line and column numbers of the character.
impl Iterator for CharIter<'_> {
    type Item = CharItem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((byte_index, ch)) = self.char_indices.next() {
            self.offset = byte_index + ch.len_utf8();
            Some(CharItem::new(byte_index, ch))
        } else {
            None
        }
    }
}

/// An iterator over characters in a string slice, yielding tuples of (char_index, char, Position).
/// The `CharIter` struct provides an iterator that tracks the position of characters in a string,
/// including their byte index, line number, and column number.
#[derive(Debug, Clone)]
pub struct CharIterWithPosition<'a> {
    char_indices: CharIndices<'a>,
    offset: usize,
    line: usize,
    column: usize,
}

impl<'a> CharIterWithPosition<'a> {
    /// Creates a new `CharIter` from the given string slice.
    pub fn new(haystack: &'a str, offset: usize) -> Self {
        let char_indices = if offset <= haystack.len() {
            haystack[offset..].char_indices()
        } else {
            haystack[haystack.len()..haystack.len()].char_indices()
        };
        CharIterWithPosition {
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

    /// Returns the next character without advancing the iterator.
    pub(crate) fn peek(&mut self) -> Option<CharItem> {
        if let Some((byte_index, ch)) = self.char_indices.clone().next() {
            let (line, column) = if ch == '\n' {
                (self.line + 1, 0)
            } else {
                (self.line, self.column + 1)
            };
            Some(CharItem::new(byte_index, ch).with_position(Position::new(line, column)))
        } else {
            None
        }
    }
}

// CharIter implements an iterator over characters in a string slice,
// yielding tuples of (char_index, char, Position) where Position
// contains the line and column numbers of the character.
impl Iterator for CharIterWithPosition<'_> {
    type Item = CharItem;

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
            Some(CharItem::new(byte_index, ch).with_position(Position::new(line, column)))
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
        let mut iter = CharIterWithPosition::new(haystack, 0);

        assert_eq!(
            iter.next(),
            Some(CharItem::new(0, 'H').with_position(Position::new(1, 1)))
        );
        assert_eq!(
            iter.next(),
            Some(CharItem::new(1, 'e').with_position(Position::new(1, 2)))
        );
        assert_eq!(
            iter.next(),
            Some(CharItem::new(2, 'l').with_position(Position::new(1, 3)))
        );
        assert_eq!(
            iter.next(),
            Some(CharItem::new(3, 'l').with_position(Position::new(1, 4)))
        );
        assert_eq!(
            iter.next(),
            Some(CharItem::new(4, 'o').with_position(Position::new(1, 5)))
        );
        assert_eq!(
            iter.next(),
            Some(CharItem::new(5, '\n').with_position(Position::new(2, 0)))
        );
        assert_eq!(
            iter.next(),
            Some(CharItem::new(6, 'W').with_position(Position::new(2, 1)))
        );
        assert_eq!(
            iter.next(),
            Some(CharItem::new(7, 'o').with_position(Position::new(2, 2)))
        );
        assert_eq!(
            iter.next(),
            Some(CharItem::new(8, 'r').with_position(Position::new(2, 3)))
        );
        assert_eq!(
            iter.next(),
            Some(CharItem::new(9, 'l').with_position(Position::new(2, 4)))
        );
        assert_eq!(
            iter.next(),
            Some(CharItem::new(10, 'd').with_position(Position::new(2, 5)))
        );
        assert_eq!(iter.next(), None);
    }
}
