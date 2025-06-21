// ScannerImpl struct and its implementation

use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct ScannerImpl {
    pub(crate) current_mode: Rc<RefCell<usize>>,
    pub(crate) modes: &'static [crate::ScannerMode],
    // For each mode, we store a map of token types to their priorities.
    priority_map: OnceCell<Vec<HashMap<usize, usize>>>,
}

impl ScannerImpl {
    /// Creates a new scanner implementation with the given modes.
    pub fn new(modes: &'static [crate::ScannerMode]) -> Self {
        ScannerImpl {
            current_mode: Rc::new(RefCell::new(0)),
            modes,
            priority_map: OnceCell::new(),
        }
    }

    /// Creates a new `FindMatches` iterator for the given haystack and offset.
    pub fn find_matches<'a, F>(
        &self,
        haystack: &'a str,
        offset: usize,
        match_function: &'static F,
    ) -> crate::internals::find_matches::FindMatches<'a, F>
    where
        F: Fn(char) -> Option<usize> + 'static,
    {
        crate::internals::find_matches::FindMatches::new(
            haystack,
            offset,
            Rc::new(RefCell::new(self.clone())),
            match_function,
        )
    }

    /// Creates a new `FindMatchesWithPosition` iterator for the given haystack and offset.
    pub fn find_matches_with_position<'h, F>(
        &self,
        haystack: &'h str,
        offset: usize,
        match_function: &'static F,
    ) -> crate::internals::find_matches::FindMatchesWithPosition<'h, F>
    where
        F: Fn(char) -> Option<usize> + 'static,
    {
        crate::internals::find_matches::FindMatchesWithPosition::new(
            haystack,
            offset,
            Rc::new(RefCell::new(self.clone())),
            match_function,
        )
    }

    /// Returns the index of the new mode based on the token type.
    /// If no transition exists for the token type, it returns `None`.
    pub fn next_mode(&self, token_type: usize) -> Option<usize> {
        self.modes[*self.current_mode.borrow()].next_mode(token_type)
    }

    /// Returns the priority of the token type in this mode.
    pub fn token_priority(&self, token_type: usize) -> Option<usize> {
        let priority_map = self.priority_map.get_or_init(|| {
            self.modes
                .iter()
                .map(|mode| {
                    mode.dfa
                        .states
                        .iter()
                        .filter_map(|state| {
                            state
                                .accept_data
                                .as_ref()
                                .map(|ad| (ad.token_type, ad.priority))
                        })
                        .collect()
                })
                .collect()
        });

        let mode_index = *self.current_mode.borrow();
        priority_map
            .get(mode_index)
            .and_then(|map| map.get(&token_type))
            .cloned()
    }

    /// Returns the name of the given mode.
    pub fn mode_name(&self, index: usize) -> Option<&'static str> {
        Some(
            self.modes
                .get(index)
                .map_or_else(|| "Unknown", |mode| mode.name),
        )
    }
}
