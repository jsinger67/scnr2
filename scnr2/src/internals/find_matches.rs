//! FindMatches struct and its implementation

use std::{cell::RefCell, rc::Rc};

use crate::{
    Dfa, Lookahead, ScannerImpl,
    internals::{
        char_iter::item::CharItem,
        char_iter::iter::CharIter,
        char_iter::iter_with_position::CharIterWithPosition,
        match_types::{Match, MatchEnd, MatchStart},
        position::{Position, Positions},
    },
};

/// A trait that defines the behavior of finding matches in a string slice using a DFA (Deterministic
/// Finite Automaton).
pub trait FindMatchesTrait {
    /// Returns the current DFA.
    fn current_dfa(&self) -> &'static Dfa;

    /// Handles the transition to a new mode based on the token type.
    fn handle_mode_transition(&self, token_type: usize);

    /// Returns the next character without advancing the iterator.
    fn peek(&mut self) -> Option<CharItem>;

    /// Returns the character class for the given character.
    fn get_disjoint_class(&self, ch: char) -> Option<usize>;

    /// Advances the character iterator to the next character.
    /// This method is used to move the iterator forward in the input string slice.
    /// It should return `true` if the iterator was advanced successfully, or `false` if it has
    /// reached the end of the input string slice.
    fn advance_char_iter(&mut self) -> bool;

    /// Saves the current character iterator state.
    fn save_char_iter(&mut self);

    /// Restores the saved character iterator state.
    fn restore_saved_char_iter(&mut self);
}

/// A structure that represents an iterator over character matches in a string slice.
#[derive(Clone)]
/**
 * Iterator over token matches in the input text.
 *
 * # Example
 * ```rust
 * use scnr2::{scanner, Match};
 * scanner! {
 *     SimpleScanner {
 *         mode INITIAL {
 *             token r"\d+" => 1;
 *         }
 *     }
 * }
 * let input = "123 456";
 * let scanner = simple_scanner::SimpleScanner::new();
 * let matches: Vec<Match> = scanner.find_matches(input, 0).collect();
 * assert_eq!(matches.len(), 2);
 * ```
 */
pub struct FindMatches<'a, F>
where
    F: Fn(char) -> Option<usize> + 'static + Clone,
{
    /// An iterator over characters in the input string slice, starting from the given offset.
    char_iter: CharIter<'a>,
    /// The creating scanner implementation, wrapped in an `Rc<RefCell>` for thread safety.
    scanner_impl: Rc<RefCell<ScannerImpl>>,
    /// A reference to the match function that returns the character class for a given character.
    match_function: &'static F,
}

impl<'a, F> FindMatches<'a, F>
where
    F: Fn(char) -> Option<usize> + 'static + Clone,
{
    /// Creates a new `FindMatches` from the given string slice and start position.
    pub(crate) fn new(
        input: &'a str,
        offset: usize,
        scanner_impl: Rc<RefCell<ScannerImpl>>,
        match_function: &'static F,
    ) -> Self {
        FindMatches {
            char_iter: CharIter::new(input, offset),
            scanner_impl,
            match_function,
        }
    }

    /// Returns the name of the current mode.
    #[inline]
    pub fn current_mode_name(&self) -> &'static str {
        let scanner_impl = self.scanner_impl.borrow();
        scanner_impl.current_mode_name()
    }

    /// Returns the name of the given mode.
    #[inline]
    pub fn mode_name(&self, index: usize) -> Option<&'static str> {
        self.scanner_impl.borrow().mode_name(index)
    }

    /// Returns the current mode index.
    #[inline]
    pub fn current_mode_index(&self) -> usize {
        self.scanner_impl.borrow().current_mode_index()
    }
}

impl<F> Iterator for FindMatches<'_, F>
where
    F: Fn(char) -> Option<usize> + 'static + Clone,
{
    type Item = Match;

    fn next(&mut self) -> Option<Match> {
        next_match(self)
    }
}

impl<F> FindMatchesTrait for FindMatches<'_, F>
where
    F: Fn(char) -> Option<usize> + 'static + Clone,
{
    /// Returns the current DFA.
    /// This method returns a reference to the DFA that is currently being used for matching.
    #[inline(always)]
    fn current_dfa(&self) -> &'static Dfa {
        let scanner_impl = self.scanner_impl.borrow();
        &scanner_impl.modes()[scanner_impl.current_mode_index()].dfa
    }

    /// Handles the transition to a new mode based on the token type.
    #[inline(always)]
    fn handle_mode_transition(&self, token_type: usize) {
        let scanner_impl = self.scanner_impl.borrow();
        scanner_impl.handle_mode_transition(token_type);
    }

    /// Returns the next character without advancing the iterator.
    #[inline(always)]
    fn peek(&mut self) -> Option<CharItem> {
        self.char_iter.peek()
    }

    /// Advances the character iterator to the next character.
    /// This method is used to move the iterator forward in the input string slice.
    /// It should return `true` if the iterator was advanced successfully, or `false` if it has
    /// reached the end of the input string slice.
    #[inline(always)]
    fn advance_char_iter(&mut self) -> bool {
        self.char_iter.next().is_some()
    }

    /// Returns the character class for the given character.
    #[inline(always)]
    fn get_disjoint_class(&self, ch: char) -> Option<usize> {
        (self.match_function)(ch)
    }

    /// Saves the current character iterator state.
    fn save_char_iter(&mut self) {
        self.char_iter.save_state();
    }

    /// Restores the saved character iterator state.
    fn restore_saved_char_iter(&mut self) {
        self.char_iter.restore_state();
    }
}

/// A structure that represents an iterator over character matches with positions in a string slice.
/// It uses the `FindMatches` struct for implementation, but includes additional position
/// information for each match.
#[derive(Clone)]
/**
 * Iterator over token matches with position information (line/column).
 *
 * # Example
 * ```rust
 * use scnr2::{scanner, Match};
 * scanner! {
 *     SimpleScanner {
 *         mode INITIAL {
 *             token r"\d+" => 1;
 *         }
 *     }
 * }
 * let input = "123\n456";
 * let scanner = simple_scanner::SimpleScanner::new();
 * let matches: Vec<Match> = scanner.find_matches_with_position(input, 0).collect();
 * assert_eq!(matches[0].positions.is_some(), true);
 * ```
 */
pub struct FindMatchesWithPosition<'a, F>
where
    F: Fn(char) -> Option<usize> + 'static + Clone,
{
    /// An iterator over characters in the input string slice, starting from the given offset.
    char_iter: CharIterWithPosition<'a>,
    /// The creating scanner implementation, wrapped in an `Rc<RefCell>` for thread safety.
    scanner_impl: Rc<RefCell<ScannerImpl>>,
    /// A reference to the match function that returns the character class for a given character.
    match_function: &'static F,
}

impl<'a, F> FindMatchesWithPosition<'a, F>
where
    F: Fn(char) -> Option<usize> + 'static + Clone,
{
    /// Creates a new `FindMatchesWithPosition` from the given string slice and start position.
    pub(crate) fn new(
        input: &'a str,
        offset: usize,
        scanner_impl: Rc<RefCell<ScannerImpl>>,
        match_function: &'static F,
    ) -> Self {
        FindMatchesWithPosition {
            char_iter: CharIterWithPosition::new(input, offset),
            scanner_impl,
            match_function,
        }
    }

    /// Returns the name of the current mode.
    #[inline]
    pub fn current_mode_name(&self) -> Option<&'static str> {
        let scanner_impl = self.scanner_impl.borrow();
        let current_mode_index = scanner_impl.current_mode_index();
        scanner_impl.mode_name(current_mode_index)
    }

    /// Returns the name of the given mode.
    #[inline]
    pub fn mode_name(&self, index: usize) -> Option<&'static str> {
        self.scanner_impl.borrow().mode_name(index)
    }

    /// Returns the current mode index.
    #[inline]
    pub fn current_mode(&self) -> usize {
        self.scanner_impl.borrow().current_mode_index()
    }
}

impl<F> Iterator for FindMatchesWithPosition<'_, F>
where
    F: Fn(char) -> Option<usize> + 'static + Clone,
{
    type Item = Match;

    fn next(&mut self) -> Option<Match> {
        next_match(self)
    }
}

impl<F> FindMatchesTrait for FindMatchesWithPosition<'_, F>
where
    F: Fn(char) -> Option<usize> + 'static + Clone,
{
    /// Returns the current DFA.
    /// This method returns a reference to the DFA that is currently being used for matching.
    #[inline(always)]
    fn current_dfa(&self) -> &'static Dfa {
        let scanner_impl = self.scanner_impl.borrow();
        &scanner_impl.modes()[scanner_impl.current_mode_index()].dfa
    }

    /// Handles the transition to a new mode based on the token type.
    #[inline(always)]
    fn handle_mode_transition(&self, token_type: usize) {
        let scanner_impl = self.scanner_impl.borrow();
        scanner_impl.handle_mode_transition(token_type);
    }

    /// Returns the next character without advancing the iterator.
    #[inline(always)]
    fn peek(&mut self) -> Option<CharItem> {
        self.char_iter.peek()
    }

    /// Advances the character iterator to the next character.
    /// This method is used to move the iterator forward in the input string slice.
    /// It should return `true` if the iterator was advanced successfully, or `false` if it has
    /// reached the end of the input string slice.
    #[inline(always)]
    fn advance_char_iter(&mut self) -> bool {
        self.char_iter.next().is_some()
    }

    /// Returns the character class for the given character.
    #[inline(always)]
    fn get_disjoint_class(&self, ch: char) -> Option<usize> {
        (self.match_function)(ch)
    }

    /// Saves the current character iterator state.
    fn save_char_iter(&mut self) {
        self.char_iter.save_state();
    }

    /// Restores the saved character iterator state.
    fn restore_saved_char_iter(&mut self) {
        self.char_iter.restore_state();
    }
}

/// Evaluates the lookahead condition for the current match.
/// This method checks if the lookahead condition is satisfied based on the
/// current match and the accept data.
/// It returns a tuple containing a boolean indicating whether the lookahead is satisfied
/// and the length of the lookahead match.
fn evaluate_lookahead<F: FindMatchesTrait + Clone>(
    mut find_matches: F,
    accept_data: &crate::AcceptData,
) -> (bool, usize) {
    match &accept_data.lookahead {
        crate::Lookahead::None => {
            unreachable!("Lookahead::None should not be evaluated here")
        }
        crate::Lookahead::Positive(dfa) => {
            // Handle positive lookahead logic here
            if let Some(ma) = find_next(&mut find_matches, dfa) {
                (true, ma.span.len())
            } else {
                (false, 0)
            }
        }
        crate::Lookahead::Negative(dfa) => {
            // Handle negative lookahead logic here
            if find_next(&mut find_matches, dfa).is_some() {
                (false, 0)
            } else {
                (true, 0)
            }
        }
    }
}

/// Returns the next match in the input, if available.
/// This method is responsible for finding the next match based on the current state of the
/// scanner implementation and the current position in the input.
/// It is used in the `next` method of the `Iterator` trait implementation.
#[inline(always)]
pub(crate) fn next_match<F: FindMatchesTrait + Clone>(find_matches: &mut F) -> Option<Match> {
    // Logic to find the next match in the input using the scanner implementation
    // and the current position in the char_iter.
    let dfa: &Dfa = find_matches.current_dfa();
    loop {
        if let Some(ma) = find_next(find_matches, dfa) {
            // If a match is found and there exists a transition to the next mode,
            // update the current mode in the scanner implementation.
            find_matches.handle_mode_transition(ma.token_type);
            return Some(ma);
        }
        // If no match, advance the iterator until a match is found
        // or the end of the input is reached.
        if !find_matches.advance_char_iter() {
            return None; // End of input reached
        }
    }
}

/// Simulates the DFA on the given input.
/// Returns a match starting at the current position. No try on next character is done.
/// The caller must do that.
///
/// If no match is found, None is returned.
#[inline(always)]
fn find_next<F: FindMatchesTrait + Clone>(find_matches: &mut F, dfa: &Dfa) -> Option<Match> {
    let mut state = 0; // Initial state of the DFA
    let mut match_start = MatchStart::default();
    let mut match_end = MatchEnd::default();
    let mut start_set = false;
    let mut end_set = false;

    find_matches.save_char_iter(); // Save the current state of the iterator

    // Iterate over characters in the input using char_iter
    while let Some(char_item) = find_matches.peek() {
        let character_class = find_matches.get_disjoint_class(char_item.ch);

        let Some(class_idx) = character_class else {
            break;
        };

        let state_data = &dfa.states[state];
        if let Some(Some(next_state)) = state_data.transitions.get(class_idx) {
            state = next_state.to;
        } else {
            break;
        };
        let state_data = &dfa.states[state];

        // Only now advance the iterator
        find_matches.advance_char_iter();

        if !start_set {
            match_start = MatchStart::new(char_item.byte_index).with_position(char_item.position);
            start_set = true;
        }

        if let Some(accept_data) = &state_data.accept_data {
            let (lookahead_satisfied, lookahead_len) =
                if !matches!(accept_data.lookahead, Lookahead::None) {
                    evaluate_lookahead(find_matches.clone(), accept_data)
                } else {
                    (true, 0)
                };
            if lookahead_satisfied {
                let new_byte_index = char_item.byte_index + lookahead_len + char_item.ch.len_utf8();
                let new_len = new_byte_index - match_start.byte_index;
                let update = !end_set || {
                    let old_len = match_end.byte_index - match_start.byte_index;
                    new_len > old_len
                        || (new_len == old_len && accept_data.priority < match_end.priority)
                };
                if update {
                    match_end =
                        MatchEnd::new(new_byte_index, accept_data.token_type, accept_data.priority)
                            .with_position(
                                char_item
                                    .position
                                    .map(|p| Position::new(p.line, p.column + 1)),
                            );
                    end_set = true;
                    find_matches.save_char_iter();
                }
            }
        }
    }

    if end_set {
        let span: crate::Span = match_start.byte_index..match_end.byte_index;
        find_matches.restore_saved_char_iter();
        Some(
            Match::new(span, match_end.token_type).with_positions(
                match_start
                    .position
                    .zip(match_end.position)
                    .map(|(start, end)| Positions::new(start, end)),
            ),
        )
    } else {
        find_matches.restore_saved_char_iter();
        None
    }
}
