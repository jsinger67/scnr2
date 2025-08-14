//! CharItem struct for representing a character and its position.

use crate::internals::position::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharItem {
    pub byte_index: usize,
    pub ch: char,
    pub position: Option<Position>,
}

impl CharItem {
    #[inline]
    pub fn new(char_index: usize, ch: char) -> Self {
        CharItem {
            byte_index: char_index,
            ch,
            position: None,
        }
    }
    #[inline]
    pub fn with_position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }
}
