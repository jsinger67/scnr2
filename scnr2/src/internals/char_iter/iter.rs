//! CharIter struct for iterating over characters in a string slice.

use crate::internals::char_iter::item::CharItem;
use std::str::CharIndices;

#[derive(Debug, Clone)]
struct SavedCharIterState<'a> {
    char_indices: CharIndices<'a>,
}

#[derive(Debug, Clone)]
pub struct CharIter<'a> {
    char_indices: CharIndices<'a>,
    saved_state: Option<SavedCharIterState<'a>>,
}

impl<'a> CharIter<'a> {
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

    pub(crate) fn peek(&mut self) -> Option<CharItem> {
        if let Some((byte_index, ch)) = self.char_indices.clone().next() {
            Some(CharItem::new(byte_index, ch))
        } else {
            None
        }
    }

    pub(crate) fn save_state(&mut self) {
        let saved_state = SavedCharIterState {
            char_indices: self.char_indices.clone(),
        };
        self.saved_state = Some(saved_state);
    }

    pub(crate) fn restore_state(&mut self) {
        if let Some(saved) = self.saved_state.take() {
            self.char_indices = saved.char_indices;
        }
    }
}

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
