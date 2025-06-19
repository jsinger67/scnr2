//! FindMatches struct and its implementation

use std::{cell::RefCell, rc::Rc};

use crate::{
    Dfa, Lookahead, ScannerImpl,
    internals::{
        char_iter::CharIter,
        match_types::{Match, MatchWithPosition},
    },
};

/// A structure that represents an iterator over character matches in a string slice.
#[derive(Debug, Clone)]
pub struct FindMatches<'a, F>
where
    F: Fn(char) -> Option<usize> + 'static,
{
    /// The input string slice in which to find matches.
    input: &'a str,
    /// The offset in the input string slice from which to start searching for matches.
    offset: usize,
    /// An iterator over characters in the input string slice, starting from the given offset.
    char_iter: CharIter<'a>,
    /// The creating scanner implementation, wrapped in an Rc<RefCell> for thread safety.
    scanner_impl: Rc<RefCell<ScannerImpl>>,
    /// A reference to the match function that returns the character class for a given character.
    match_function: &'static F,
}

/// Helper structures to manage the start and end of matches with their positions.
struct MatchStart {
    byte_index: usize,
    position: crate::internals::position::Position,
}

/// Helper structure to manage the end of matches with their positions, token type, and priority.
struct MatchEnd {
    byte_index: usize,
    position: crate::internals::position::Position,
    token_type: usize,
    priority: usize,
}

impl<'a, F> FindMatches<'a, F>
where
    F: Fn(char) -> Option<usize> + 'static,
{
    /// Creates a new `FindIter` from the given string slice and start position.
    pub(crate) fn new(
        input: &'a str,
        offset: usize,
        scanner_impl: Rc<RefCell<ScannerImpl>>,
        match_function: &'static F,
    ) -> Self {
        FindMatches {
            input,
            offset,
            char_iter: CharIter::new(input, offset),
            scanner_impl,
            match_function,
        }
    }

    /// Returns the next match in the haystack, if available.
    /// This method is responsible for finding the next match based on the current state of the
    /// scanner implementation and the current position in the haystack.
    /// It is used in the `next` method of the `Iterator` trait implementation.
    pub(crate) fn next_match(&mut self) -> Option<MatchWithPosition> {
        // Logic to find the next match in the haystack using the scanner implementation
        // and the current position in the char_iter.
        let dfa: &Dfa = {
            let scanner_impl = self.scanner_impl.borrow();
            &scanner_impl.modes[*scanner_impl.current_mode.borrow()].dfa
        };
        let ma = self.find_next(dfa);
        // If a match is found and there exists a transition to the next mode,
        // update the current mode in the scanner implementation.
        if let Some(ma) = &ma {
            let scanner_impl = self.scanner_impl.borrow();
            if let Some(next_mode) = scanner_impl.next_mode(ma.token_type) {
                *scanner_impl.current_mode.borrow_mut() = next_mode;
            }
        }
        ma
    }

    /// Simulates the DFA on the given input.
    /// Returns a match starting at the current position. No try on next character is done.
    /// The caller must do that.
    ///
    /// If no match is found, None is returned.
    fn find_next(&mut self, dfa: &Dfa) -> Option<MatchWithPosition> {
        let mut state = 0; // Initial state of the DFA
        let mut match_start: Option<MatchStart> = None;

        let mut match_end: Option<MatchEnd> = None;

        // Iterate over characters in the haystack using char_iter
        while let Some((byte_index, ch, position)) = self.char_iter.peek() {
            let character_class = (self.match_function)(ch);
            let state_data = &dfa.states[state];

            let Some(class_idx) = character_class else {
                break;
            };
            let Ok(transition_index) = state_data
                .transitions
                .binary_search_by_key(&class_idx, |t| t.char_class)
            else {
                break;
            };

            // Only now advance the iterator
            self.char_iter.next();

            if match_start.is_none() {
                match_start = Some(MatchStart {
                    byte_index,
                    position,
                });
            }

            state = state_data.transitions[transition_index].to;
            let state_data = &dfa.states[state];

            if let Some(accept_data) = &state_data.accept_data {
                let (lookahead_satisfied, lookahead_len) =
                    if !matches!(accept_data.lookahead, Lookahead::None) {
                        let find_matches = FindMatches::new(
                            self.input,
                            self.offset,
                            self.scanner_impl.clone(),
                            self.match_function,
                        );
                        Self::evaluate_lookahead(find_matches, accept_data)
                    } else {
                        (true, 0)
                    };
                if lookahead_satisfied {
                    let match_start = match_start.as_ref().unwrap();
                    let new_byte_index = byte_index + lookahead_len + ch.len_utf8();
                    let new_len = new_byte_index - match_start.byte_index;
                    let update = match &match_end {
                        Some(me) => {
                            let old_len = me.byte_index - match_start.byte_index;
                            new_len > old_len
                                || (new_len == old_len && accept_data.priority < me.priority)
                        }
                        None => true,
                    };
                    if update {
                        match_end = Some(MatchEnd {
                            byte_index: new_byte_index,
                            position,
                            token_type: accept_data.token_type,
                            priority: accept_data.priority,
                        });
                    }
                }
            }
        }

        if let Some(match_end) = match_end {
            let match_start = match_start.unwrap();
            let span: crate::Span = match_start.byte_index..match_end.byte_index;
            Some(MatchWithPosition::new(
                Match::new(span, match_end.token_type),
                match_start.position,
                match_end.position,
            ))
        } else {
            None
        }
    }

    /// Evaluates the lookahead condition for the current match.
    /// This method checks if the lookahead condition is satisfied based on the
    /// current match and the accept data.
    /// It returns a tuple containing a boolean indicating whether the lookahead is satisfied
    /// and the length of the lookahead match.
    fn evaluate_lookahead(
        mut find_matches: FindMatches<'_, F>,
        accept_data: &crate::AcceptData,
    ) -> (bool, usize) {
        match &accept_data.lookahead {
            crate::Lookahead::None => {
                unreachable!("Lookahead::None should not be evaluated here")
            }
            crate::Lookahead::Positive(dfa) => {
                // Handle positive lookahead logic here
                if let Some(ma) = find_matches.find_next(dfa) {
                    (true, ma.span.len())
                } else {
                    (false, 0)
                }
            }
            crate::Lookahead::Negative(dfa) => {
                // Handle negative lookahead logic here
                if find_matches.find_next(dfa).is_some() {
                    (false, 0)
                } else {
                    (true, 0)
                }
            }
        }
    }
}

impl<F> Iterator for FindMatches<'_, F>
where
    F: Fn(char) -> Option<usize> + 'static,
{
    type Item = Match;

    fn next(&mut self) -> Option<Match> {
        self.next_match().map(|m| Match {
            span: m.span,
            token_type: m.token_type,
        })
    }
}

/// A structure that represents an iterator over character matches with positions in a string slice.
/// It uses the `FindMatches` struct for implementation, but includes additional position
/// information for each match.
#[derive(Debug, Clone)]
pub struct FindMatchesWithPosition<'a, F>
where
    F: Fn(char) -> Option<usize> + 'static,
{
    find_matches: FindMatches<'a, F>,
}

impl<'a, F> FindMatchesWithPosition<'a, F>
where
    F: Fn(char) -> Option<usize> + 'static,
{
    /// Creates a new `FindMatchesWithPosition` from the given string slice and start position.
    pub(crate) fn new(
        haystack: &'a str,
        offset: usize,
        scanner_impl: Rc<RefCell<ScannerImpl>>,
        match_function: &'static F,
    ) -> Self {
        FindMatchesWithPosition {
            find_matches: FindMatches::new(haystack, offset, scanner_impl, match_function),
        }
    }
}

impl<F> Iterator for FindMatchesWithPosition<'_, F>
where
    F: Fn(char) -> Option<usize> + 'static,
{
    type Item = MatchWithPosition;

    fn next(&mut self) -> Option<MatchWithPosition> {
        self.find_matches.next_match()
    }
}
