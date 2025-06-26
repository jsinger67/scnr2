// ScannerImpl struct and its implementation

use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use crate::{
    Transition,
    internals::find_matches::{FindMatches, FindMatchesWithPosition},
};

#[derive(Debug, Clone)]
pub struct ScannerImpl {
    /// The current mode index, wrapped in a `RefCell` for interior mutability.
    pub(crate) current_mode: Rc<RefCell<usize>>,
    /// The mode stack
    pub(crate) mode_stack: Rc<RefCell<Vec<usize>>>,
    pub(crate) modes: &'static [crate::ScannerMode],
    // For each mode we store a map of token types to their transitions.
    transition_map: OnceCell<Vec<HashMap<usize, Transition>>>,
}

impl ScannerImpl {
    /// Creates a new scanner implementation with the given modes.
    pub fn new(modes: &'static [crate::ScannerMode]) -> Self {
        ScannerImpl {
            current_mode: Rc::new(RefCell::new(0)),
            mode_stack: Rc::new(RefCell::new(vec![])),
            modes,
            transition_map: OnceCell::new(),
        }
    }

    /// Creates a new `FindMatches` iterator for the given input and offset.
    pub fn find_matches<'a, F>(
        &self,
        input: &'a str,
        offset: usize,
        match_function: &'static F,
    ) -> FindMatches<'a, F>
    where
        F: Fn(char) -> Option<usize> + 'static + Clone,
    {
        FindMatches::new(
            input,
            offset,
            Rc::new(RefCell::new(self.clone())),
            match_function,
        )
    }

    /// Creates a new `FindMatchesWithPosition` iterator for the given input and offset.
    pub fn find_matches_with_position<'h, F>(
        &self,
        input: &'h str,
        offset: usize,
        match_function: &'static F,
    ) -> FindMatchesWithPosition<'h, F>
    where
        F: Fn(char) -> Option<usize> + 'static + Clone,
    {
        FindMatchesWithPosition::new(
            input,
            offset,
            Rc::new(RefCell::new(self.clone())),
            match_function,
        )
    }

    #[inline(always)]
    pub fn handle_mode_transition(&self, token_type: usize) {
        let mode_index = *self.current_mode.borrow();
        if let Some(transition) = self.transition_for_token_type(token_type) {
            match transition {
                crate::Transition::SetMode(_, m) => {
                    *self.current_mode.borrow_mut() = *m;
                }
                crate::Transition::PushMode(_, m) => {
                    self.mode_stack.borrow_mut().push(mode_index);
                    *self.current_mode.borrow_mut() = *m;
                }
                crate::Transition::PopMode(_) => {
                    if let Some(previous_mode_index) = self.mode_stack.borrow_mut().pop() {
                        *self.current_mode.borrow_mut() = previous_mode_index;
                    } else {
                        // If the stack is empty, we stay in the current mode.
                        // This is a no-op, but it ensures we don't panic.
                    }
                }
            }
        }
    }

    /// Returns the transition for the given token type in the current mode.
    fn transition_for_token_type(&self, token_type: usize) -> Option<&Transition> {
        let transition_map = self.transition_map.get_or_init(|| {
            self.modes
                .iter()
                .map(|mode| {
                    mode.transitions
                        .iter()
                        .map(|transition| (transition.token_type(), transition.clone()))
                        .collect()
                })
                .collect()
        });

        let mode_index = *self.current_mode.borrow();
        transition_map
            .get(mode_index)
            .and_then(|map| map.get(&token_type))
    }

    /// Returns the current mode index.
    pub fn current_mode_index(&self) -> usize {
        *self.current_mode.borrow()
    }

    /// Returns the name of the given mode.
    pub fn mode_name(&self, index: usize) -> Option<&'static str> {
        Some(
            self.modes
                .get(index)
                .map_or_else(|| "Unknown", |mode| mode.name),
        )
    }

    /// returns the name of the current mode.
    pub fn current_mode_name(&self) -> &'static str {
        self.mode_name(self.current_mode_index())
            .unwrap_or("Unknown")
    }
}
