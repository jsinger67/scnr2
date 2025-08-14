//! CharIterWithPosition struct for iterating over characters with position tracking.

use crate::internals::char_iter::item::CharItem;
use crate::internals::position::Position;
use std::str::CharIndices;

#[derive(Debug, Clone)]
struct SavedCharIterWithPositionState<'a> {
    char_indices: CharIndices<'a>,
    line: usize,
    column: usize,
}

#[derive(Debug, Clone)]
pub struct CharIterWithPosition<'a> {
    char_indices: CharIndices<'a>,
    line: usize,
    column: usize,
    last_char: char,
    saved_state: Option<SavedCharIterWithPositionState<'a>>,
}

impl<'a> CharIterWithPosition<'a> {
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

    pub fn position(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    pub(crate) fn peek(&mut self) -> Option<CharItem> {
        if let Some((byte_index, ch)) = self.char_indices.clone().next() {
            let (line, column) = if self.last_char == '\n' {
                (self.line + 1, 1)
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

impl Iterator for CharIterWithPosition<'_> {
    type Item = CharItem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((byte_index, ch)) = self.char_indices.next() {
            let (line, column) = if self.last_char == '\n' {
                (self.line + 1, 1)
            } else {
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
