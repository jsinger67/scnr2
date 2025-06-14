// FindMatches struct and its implementation

#[derive(Debug, Clone)]
pub struct FindMatches<'a> {
    haystack: &'a str,
    char_iter: super::CharIter<'a>,
    offset: usize,
}

impl<'a> FindMatches<'a> {
    /// Creates a new `FindIter` from the given string slice and start position.
    pub fn new(haystack: &'a str, offset: usize) -> Self {
        FindMatches {
            haystack,
            char_iter: super::CharIter::new(haystack, offset),
            offset,
        }
    }
}
