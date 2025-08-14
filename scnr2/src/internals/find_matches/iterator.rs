//! Iterators for finding matches in a string slice.

use super::traits::FindMatchesTrait;
use crate::{
    Dfa,
    internals::{
        char_iter::{CharItem, CharIter, CharIterWithPosition},
        match_types::Match,
        scanner_impl::ScannerImpl,
    },
};
use std::{cell::RefCell, rc::Rc};

/// Iterator over token matches in the input text.
#[derive(Clone)]
pub struct FindMatches<'a, F>
where
    F: Fn(char) -> Option<usize> + 'static + Clone,
{
    char_iter: CharIter<'a>,
    scanner_impl: Rc<RefCell<ScannerImpl>>,
    match_function: &'static F,
}

impl<'a, F> FindMatches<'a, F>
where
    F: Fn(char) -> Option<usize> + 'static + Clone,
{
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

    /// Returns the name of the current scanner mode.
    ///
    /// # Returns
    /// The name of the current mode as a static string slice.
    pub fn current_mode_name(&self) -> &'static str {
        let scanner_impl = self.scanner_impl.borrow();
        scanner_impl.current_mode_name()
    }

    /// Returns the name of the scanner mode at the given index.
    ///
    /// # Arguments
    /// * `index` - The mode index.
    ///
    /// # Returns
    /// The name of the mode as a static string slice, or `None` if the index is invalid.
    pub fn mode_name(&self, index: usize) -> Option<&'static str> {
        self.scanner_impl.borrow().mode_name(index)
    }

    /// Returns the index of the current scanner mode.
    ///
    /// # Returns
    /// The index of the current mode.
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
        super::matching::next_match(self)
    }
}

impl<F> FindMatchesTrait for FindMatches<'_, F>
where
    F: Fn(char) -> Option<usize> + 'static + Clone,
{
    fn current_dfa(&self) -> &'static Dfa {
        let scanner_impl = self.scanner_impl.borrow();
        &scanner_impl.modes()[scanner_impl.current_mode_index()].dfa
    }

    fn handle_mode_transition(&self, token_type: usize) {
        let scanner_impl = self.scanner_impl.borrow();
        scanner_impl.handle_mode_transition(token_type);
    }

    fn peek(&mut self) -> Option<CharItem> {
        self.char_iter.peek()
    }

    fn advance_char_iter(&mut self) -> bool {
        self.char_iter.next().is_some()
    }

    fn get_disjoint_class(&self, ch: char) -> Option<usize> {
        (self.match_function)(ch)
    }

    fn save_char_iter(&mut self) {
        self.char_iter.save_state();
    }

    fn restore_saved_char_iter(&mut self) {
        self.char_iter.restore_state();
    }
}

/// Iterator over token matches with position information (line/column).
#[derive(Clone)]
pub struct FindMatchesWithPosition<'a, F>
where
    F: Fn(char) -> Option<usize> + 'static + Clone,
{
    char_iter: CharIterWithPosition<'a>,
    scanner_impl: Rc<RefCell<ScannerImpl>>,
    match_function: &'static F,
}

impl<'a, F> FindMatchesWithPosition<'a, F>
where
    F: Fn(char) -> Option<usize> + 'static + Clone,
{
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

    /// Returns the name of the current scanner mode, if available.
    ///
    /// # Returns
    /// The name of the current mode as a static string slice, or `None` if unavailable.
    pub fn current_mode_name(&self) -> Option<&'static str> {
        let scanner_impl = self.scanner_impl.borrow();
        let current_mode_index = scanner_impl.current_mode_index();
        scanner_impl.mode_name(current_mode_index)
    }

    /// Returns the name of the scanner mode at the given index, if available.
    ///
    /// # Arguments
    /// * `index` - The mode index.
    ///
    /// # Returns
    /// The name of the mode as a static string slice, or `None` if the index is invalid.
    pub fn mode_name(&self, index: usize) -> Option<&'static str> {
        self.scanner_impl.borrow().mode_name(index)
    }

    /// Returns the index of the current scanner mode.
    ///
    /// # Returns
    /// The index of the current mode.
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
        super::matching::next_match(self)
    }
}

impl<F> FindMatchesTrait for FindMatchesWithPosition<'_, F>
where
    F: Fn(char) -> Option<usize> + 'static + Clone,
{
    fn current_dfa(&self) -> &'static Dfa {
        let scanner_impl = self.scanner_impl.borrow();
        &scanner_impl.modes()[scanner_impl.current_mode_index()].dfa
    }

    fn handle_mode_transition(&self, token_type: usize) {
        let scanner_impl = self.scanner_impl.borrow();
        scanner_impl.handle_mode_transition(token_type);
    }

    fn peek(&mut self) -> Option<CharItem> {
        self.char_iter.peek()
    }

    fn advance_char_iter(&mut self) -> bool {
        self.char_iter.next().is_some()
    }

    fn get_disjoint_class(&self, ch: char) -> Option<usize> {
        (self.match_function)(ch)
    }

    fn save_char_iter(&mut self) {
        self.char_iter.save_state();
    }

    fn restore_saved_char_iter(&mut self) {
        self.char_iter.restore_state();
    }
}
