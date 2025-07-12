//! ScannerImpl struct and its implementation

use std::{
    cell::{Cell, OnceCell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use log::trace;

use crate::{
    Transition,
    internals::find_matches::{FindMatches, FindMatchesWithPosition},
};

pub struct ScannerImpl {
    /// The current mode index, wrapped in a `RefCell` for interior mutability.
    pub(crate) current_mode: Cell<usize>,
    /// The mode stack.
    pub(crate) mode_stack: Cell<Vec<usize>>,
    /// The scanner modes available to this scanner implementation.
    pub(crate) modes: &'static [crate::ScannerMode],
    /// For each mode, stores a map of token types to their transitions.
    transition_map: OnceCell<Vec<HashMap<usize, Transition>>>,
}

impl ScannerImpl {
    /// Creates a new scanner implementation with the given modes.
    pub fn new(modes: &'static [crate::ScannerMode]) -> Self {
        ScannerImpl {
            current_mode: Cell::new(0),
            mode_stack: Cell::new(vec![]),
            modes,
            transition_map: OnceCell::new(),
        }
    }

    /// Returns a reference to the modes of this scanner implementation.
    #[inline(always)]
    pub fn modes(&self) -> &'static [crate::ScannerMode] {
        self.modes
    }

    /// Creates a new `FindMatches` iterator for the given input and offset.
    pub fn find_matches<'a, F>(
        scanner_impl: Rc<RefCell<Self>>,
        input: &'a str,
        offset: usize,
        match_function: &'static F,
    ) -> FindMatches<'a, F>
    where
        F: Fn(char) -> Option<usize> + 'static + Clone,
    {
        FindMatches::new(input, offset, scanner_impl, match_function)
    }

    /// Creates a new `FindMatchesWithPosition` iterator for the given input and offset.
    pub fn find_matches_with_position<'h, F>(
        scanner_impl: Rc<RefCell<Self>>,
        input: &'h str,
        offset: usize,
        match_function: &'static F,
    ) -> FindMatchesWithPosition<'h, F>
    where
        F: Fn(char) -> Option<usize> + 'static + Clone,
    {
        FindMatchesWithPosition::new(input, offset, scanner_impl, match_function)
    }

    #[inline(always)]
    pub fn handle_mode_transition(&self, token_type: usize) {
        let mode_index = self.current_mode.get();
        if let Some(transition) = self.transition_for_token_type(token_type) {
            match transition {
                crate::Transition::SetMode(_, m) => {
                    trace!("Setting mode to {}", m);
                    self.current_mode.set(*m);
                }
                crate::Transition::PushMode(_, m) => {
                    trace!(
                        "Pushing mode {} onto stack, switching to {}",
                        mode_index,
                        self.mode_name(*m).unwrap_or("UNKNOWN")
                    );
                    let mut mode_stack = self.mode_stack.take();
                    mode_stack.push(mode_index);
                    self.mode_stack.set(mode_stack);
                    self.current_mode.set(*m);
                }
                crate::Transition::PopMode(_) => {
                    let mut mode_stack = self.mode_stack.take();
                    if let Some(previous_mode_index) = mode_stack.pop() {
                        self.mode_stack.set(mode_stack);
                        trace!(
                            "Popping mode from stack, switching back to {}",
                            self.mode_name(previous_mode_index).unwrap_or("UNKNOWN")
                        );
                        self.current_mode.set(previous_mode_index);
                    } else {
                        trace!(
                            "Popping mode from stack, but stack is empty. Staying in current mode."
                        );
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

        let mode_index = self.current_mode.get();
        transition_map
            .get(mode_index)
            .and_then(|map| map.get(&token_type))
    }

    /// Returns the current mode index.
    #[inline]
    pub fn current_mode_index(&self) -> usize {
        self.current_mode.get()
    }

    /// Returns the name of the given mode, or "Unknown" if the index is out of bounds.
    #[inline]
    pub fn mode_name(&self, index: usize) -> Option<&'static str> {
        Some(
            self.modes
                .get(index)
                .map_or_else(|| "Unknown", |mode| mode.name),
        )
    }

    /// Returns the name of the current mode.
    #[inline]
    pub fn current_mode_name(&self) -> &'static str {
        self.mode_name(self.current_mode_index())
            .unwrap_or("Unknown")
    }
}
