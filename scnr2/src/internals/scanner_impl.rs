//! ScannerImpl struct and its implementation

use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use log::trace;

use crate::{
    Transition,
    internals::find_matches::{FindMatches, FindMatchesWithPosition},
};

#[derive(Debug, Clone)]
pub struct ScannerImpl {
    /// The current mode index, wrapped in a `RefCell` for interior mutability.
    pub(crate) current_mode: Rc<RefCell<usize>>,
    /// The mode stack.
    pub(crate) mode_stack: Rc<RefCell<Vec<usize>>>,
    /// The scanner modes available to this scanner implementation.
    pub(crate) modes: &'static [crate::ScannerMode],
    /// For each mode, stores a map of token types to their transitions.
    /// The key is the token first token type number of a possible sequence of token types,
    /// and the value is a tuple containing the sequence of token type numbers and the transition.
    transition_map: OnceCell<Vec<HashMap<usize, Transition>>>,
    /// Transitions that are currently under progress.
    /// This is used to handle transitions that are not yet completed because there need to be a
    /// sequence of token types matched before the transition can be completed.
    transitions_under_progress: Rc<RefCell<Vec<Transition>>>,
}

impl ScannerImpl {
    /// Creates a new scanner implementation with the given modes.
    pub fn new(modes: &'static [crate::ScannerMode]) -> Self {
        ScannerImpl {
            current_mode: Rc::new(RefCell::new(0)),
            mode_stack: Rc::new(RefCell::new(vec![])),
            modes,
            transition_map: OnceCell::new(),
            transitions_under_progress: Rc::new(RefCell::new(vec![])),
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

    /// Handles a mode transition for the given token type.
    /// All transitions that are currently under progress are checked first.
    /// If a transition is found that can be completed with the given token type,
    /// it is completed and the current mode is updated accordingly.
    #[inline(always)]
    pub fn handle_mode_transition(&self, token_type: usize) {
        // First handle transitions under progress.
        let mut transitions_under_progress = self.transitions_under_progress.borrow_mut();

        // Iterate over transitions under progress and retain only the once that can be possibly
        // completed.
        transitions_under_progress
            .retain(|transition| transition.token_types().first() == Some(&token_type));

        let mode_switched_or_sequence_updated = !transitions_under_progress.is_empty();

        // Iterate over transitions under progress and complete them if possible.
        for t in transitions_under_progress.iter_mut() {
            let remaining_token_types = &t.token_types()[1..];
            if remaining_token_types.is_empty() {
                // If there are no remaining token types, we can complete the transition.
                match t {
                    crate::Transition::SetMode(_, m) => {
                        trace!("Setting mode to {}", m);
                        *self.current_mode.borrow_mut() = *m;
                    }
                    crate::Transition::PushMode(_, m) => {
                        let mode_index = *self.current_mode.borrow();
                        trace!(
                            "Pushing current mode {} onto stack and switching to mode {}",
                            mode_index, m
                        );
                        self.mode_stack.borrow_mut().push(mode_index);
                        *self.current_mode.borrow_mut() = *m;
                    }
                    crate::Transition::PopMode(_) => {
                        trace!("Popping mode from stack");
                        if let Some(previous_mode_index) = self.mode_stack.borrow_mut().pop() {
                            trace!(
                                "Popped mode {}, switching back to {}",
                                previous_mode_index,
                                self.mode_name(previous_mode_index).unwrap_or("UNKNOWN")
                            );
                            *self.current_mode.borrow_mut() = previous_mode_index;
                        } else {
                            trace!("Mode stack is empty, staying in current mode.");
                        }
                    }
                }
            } else {
                // If there are remaining token types, we update the transition with the new sequence.
                t.set_token_types(remaining_token_types);
            }
        }

        if mode_switched_or_sequence_updated {
            // If we switched modes or updated the sequence, we ignore possible incoming transitions.
            trace!(
                "Handled transitions under progress, current mode is now {}",
                self.current_mode_name()
            );
            return;
        }

        // If there are no transitions under progress, we can handle incoming transitions.
        if let Some(transition) = self.transition_for_token_type(token_type) {
            let mode_index = *self.current_mode.borrow();
            match transition {
                crate::Transition::SetMode(s, m) => {
                    let remaining_token_types = &s[1..];
                    if remaining_token_types.is_empty() {
                        trace!("Setting mode to {}", m);
                        *self.current_mode.borrow_mut() = m;
                    } else {
                        trace!(
                            "Transitioning to mode {} with remaining token types: {:?}",
                            m, remaining_token_types
                        );
                        // If there are remaining token types, we store the transition for later.
                        self.transitions_under_progress
                            .borrow_mut()
                            .push(transition.with_token_types(remaining_token_types));
                    }
                }
                crate::Transition::PushMode(s, m) => {
                    let remaining_token_types = &s[1..];
                    if remaining_token_types.is_empty() {
                        trace!(
                            "Pushing current mode {} onto stack and switching to mode {}",
                            mode_index, m
                        );
                        self.mode_stack.borrow_mut().push(mode_index);
                        *self.current_mode.borrow_mut() = m;
                    } else {
                        trace!(
                            "Transitioning to mode {} with remaining token types: {:?}",
                            m, remaining_token_types
                        );
                        // If there are remaining token types, we store the transition for later.
                        self.transitions_under_progress
                            .borrow_mut()
                            .push(transition.with_token_types(remaining_token_types));
                    }
                }
                crate::Transition::PopMode(s) => {
                    let remaining_token_types = &s[1..];
                    if remaining_token_types.is_empty() {
                        trace!("Popping mode from stack");
                        // If there are no remaining token types, we pop the mode from the stack.
                        if let Some(previous_mode_index) = self.mode_stack.borrow_mut().pop() {
                            trace!(
                                "Popped mode {}, switching back to {}",
                                previous_mode_index,
                                self.mode_name(previous_mode_index).unwrap_or("UNKNOWN")
                            );
                            *self.current_mode.borrow_mut() = previous_mode_index;
                        } else {
                            trace!("Mode stack is empty, staying in current mode.");
                            // If the stack is empty, we stay in the current mode.
                            // This is a no-op, but it ensures we don't panic.
                        }
                    } else {
                        trace!(
                            "Transitioning to pop mode with remaining token types: {:?}",
                            remaining_token_types
                        );
                        // If there are remaining token types, we store the transition for later.
                        self.transitions_under_progress
                            .borrow_mut()
                            .push(transition.with_token_types(remaining_token_types));
                    }
                }
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

        let mode_index = *self.current_mode.borrow();
        transition_map
            .get(mode_index)
            .and_then(|map| map.get(&token_type).cloned())
    }

    /// Returns the current mode index.
    pub fn current_mode_index(&self) -> usize {
        *self.current_mode.borrow()
    }

    /// Returns the name of the given mode, or "Unknown" if the index is out of bounds.
    pub fn mode_name(&self, index: usize) -> Option<&'static str> {
        Some(
            self.modes
                .get(index)
                .map_or_else(|| "Unknown", |mode| mode.name),
        )
    }

    /// Returns the name of the current mode.
    pub fn current_mode_name(&self) -> &'static str {
        self.mode_name(self.current_mode_index())
            .unwrap_or("Unknown")
    }
}
