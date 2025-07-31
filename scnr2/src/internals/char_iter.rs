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
    /// Creates a new `CharItem` with the given byte index and character.
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

#[derive(Debug, Clone)]
struct SavedCharIterState<'a> {
    char_indices: CharIndices<'a>,
}

/// An iterator over characters in a string slice, yielding `CharItem` objects.
/// The `CharIter` struct provides an iterator that tracks the byte index of characters in a string.
#[derive(Debug, Clone)]
pub struct CharIter<'a> {
    char_indices: CharIndices<'a>,
    saved_state: Option<SavedCharIterState<'a>>, // To save the state of line and column
}

impl<'a> CharIter<'a> {
    /// Creates a new `CharIter` from the given string slice.
    pub fn new(input: &'a str, offset: usize) -> Self {
        let char_indices = if offset <= input.len() {
            input[offset..].char_indices()
        } else {
            input[input.len()..input.len()].char_indices()
        };
        CharIter {
            char_indices,
            saved_state: None,
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

    pub(crate) fn save_state(&mut self) {
        // Save the current state of the iterator
        let saved_state = SavedCharIterState {
            char_indices: self.char_indices.clone(),
        };
        self.saved_state = Some(saved_state);
    }

    pub(crate) fn restore_state(&mut self) {
        // Restore the saved state of the iterator
        if let Some(saved) = self.saved_state.take() {
            self.char_indices = saved.char_indices;
        }
    }
}

// CharIter implements an iterator over characters in a string slice,
// yielding CharItem objects with byte index and character.
impl Iterator for CharIter<'_> {
    type Item = CharItem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((byte_index, ch)) = self.char_indices.next() {
            Some(CharItem::new(byte_index, ch))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct SavedCharIterWithPositionState<'a> {
    char_indices: CharIndices<'a>,
    line: usize,
    column: usize,
}

/// An iterator over characters in a string slice, yielding `CharItem` objects with position tracking.
/// The `CharIterWithPosition` struct provides an iterator that tracks the position of characters in a string,
/// including their byte index, line number, and column number.
#[derive(Debug, Clone)]
pub struct CharIterWithPosition<'a> {
    char_indices: CharIndices<'a>,
    line: usize,
    column: usize,
    last_char: char,
    saved_state: Option<SavedCharIterWithPositionState<'a>>, // To save the state of line and column
}

impl<'a> CharIterWithPosition<'a> {
    /// Creates a new `CharIterWithPosition` from the given string slice.
    pub fn new(input: &'a str, offset: usize) -> Self {
        let char_indices = if offset <= input.len() {
            input[offset..].char_indices()
        } else {
            input[input.len()..input.len()].char_indices()
        };
        CharIterWithPosition {
            char_indices,
            line: 1,
            column: 0,
            last_char: '\0',
            saved_state: None,
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
                // Switching to the next line is done in the next call to `next()`
                (self.line, self.column + 1)
            } else {
                (self.line, self.column + 1)
            };
            Some(CharItem::new(byte_index, ch).with_position(Position::new(line, column)))
        } else {
            None
        }
    }

    pub(crate) fn save_state(&mut self) {
        let saved_state = SavedCharIterWithPositionState {
            char_indices: self.char_indices.clone(),
            line: self.line,
            column: self.column,
        };
        self.saved_state = Some(saved_state);
    }

    pub(crate) fn restore_state(&mut self) {
        if let Some(saved) = self.saved_state.take() {
            self.char_indices = saved.char_indices;
            self.line = saved.line;
            self.column = saved.column;
        }
    }
}

// CharIterWithPosition implements an iterator over characters in a string slice,
// yielding CharItem objects with byte index, character, and position information.
impl Iterator for CharIterWithPosition<'_> {
    type Item = CharItem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((byte_index, ch)) = self.char_indices.next() {
            let (line, column) = if ch == '\n' {
                (self.line, self.column + 1) // Do not increment line here
            } else if self.last_char == '\n' {
                // If the last character was a newline, reset column to 1 and increment line
                (self.line + 1, 1)
            } else {
                // Otherwise, increment the column
                (self.line, self.column + 1)
            };
            self.last_char = ch;
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
        let input = "Hello\nWorld";
        let mut iter = CharIterWithPosition::new(input, 0);

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
            Some(CharItem::new(5, '\n').with_position(Position::new(1, 6)))
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
