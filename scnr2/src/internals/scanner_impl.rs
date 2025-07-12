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
    modes: &'static [crate::ScannerMode],
    /// For each mode, stores a map of token types to their transitions.
    /// The key is the token first token type number of a possible sequence of token types,
    /// and the value is a tuple containing the sequence of token type numbers and the transition.
    transition_map: OnceCell<Vec<HashMap<usize, Transition>>>,
    /// Transitions that are currently under progress.
    /// This is used to handle transitions that are not yet completed because there need to be a
    /// sequence of token types matched before the transition can be completed.
    transitions_under_progress: Cell<Vec<Transition>>,
}

impl ScannerImpl {
    /// Creates a new scanner implementation with the given modes.
    pub fn new(modes: &'static [crate::ScannerMode]) -> Self {
        ScannerImpl {
            current_mode: Cell::new(0),
            mode_stack: Cell::new(vec![]),
            modes,
            transition_map: OnceCell::new(),
            transitions_under_progress: Cell::new(vec![]),
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

    /// Executes a transition, updating mode and stack as needed.
    fn execute_transition(&self, transition: &Transition) {
        match transition {
            crate::Transition::SetMode(_, m) => {
                trace!(
                    "Setting mode to {} ({})",
                    self.mode_name(*m).unwrap_or("Unknown"),
                    m
                );
                self.current_mode.set(*m);
            }
            crate::Transition::PushMode(_, m) => {
                let mode_index = self.current_mode.get();
                trace!(
                    "Pushing current mode {} onto stack and switching to mode {}",
                    mode_index, m
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
                        "Popping mode {}, switching back to {}",
                        previous_mode_index,
                        self.mode_name(previous_mode_index).unwrap_or("UNKNOWN")
                    );
                    self.current_mode.set(previous_mode_index);
                } else {
                    trace!("Mode stack is empty, staying in current mode.");
                }
            }
        }
    }

    /// Handles a mode transition for the given token type.
    /// All transitions that are currently under progress are checked first.
    /// If a transition is found that can be completed with the given token type,
    /// it is completed and the current mode is updated accordingly.
    #[inline(always)]
    pub fn handle_mode_transition(&self, token_type: usize) {
        trace!(
            "Handling mode transition for token type {} in mode {}",
            token_type,
            self.current_mode_name()
        );
        // First handle transitions under progress.
        let mut transitions_to_keep = Vec::new();
        let mut handled = false;

        {
            let mut transitions = self.transitions_under_progress.take();
            for mut t in transitions.drain(..) {
                if t.token_types().first() == Some(&token_type) {
                    let remaining = &t.token_types()[1..];
                    if remaining.is_empty() {
                        self.execute_transition(&t);
                    } else {
                        t.set_token_types(remaining);
                        transitions_to_keep.push(t);
                    }
                    handled = true;
                }
            }
            self.transitions_under_progress.set(transitions_to_keep);
        }

        if handled {
            // Ignore incoming transitions if a transition under progress was handled.
            // This prevents unwanted interference with the current transition.
            return;
        }

        if let Some(transition) = self.transition_for_token_type(token_type) {
            let remaining = &transition.token_types()[1..];
            if remaining.is_empty() {
                self.execute_transition(&transition);
            } else {
                trace!("Transitioning with remaining token types: {:?}", remaining);
                let mut transitions = self.transitions_under_progress.take();
                transitions.push(transition.with_token_types(remaining));
                self.transitions_under_progress.set(transitions);
            }
        } else {
            trace!("No transition found for token type {}", token_type);
        }
    }

    /// Returns the transition for the given token type in the current mode.
    fn transition_for_token_type(&self, token_type: usize) -> Option<Transition> {
        let transition_map = self.transition_map.get_or_init(|| {
            self.modes
                .iter()
                .map(|mode| {
                    mode.transitions
                        .iter()
                        .map(|transition| (transition.token_types()[0], transition.clone()))
                        .collect()
                })
                .collect()
        });

        let mode_index = self.current_mode.get();
        transition_map
            .get(mode_index)
            .and_then(|map| map.get(&token_type).cloned())
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
