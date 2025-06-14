// CharIter struct and its implementation
use std::str::CharIndices;

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
